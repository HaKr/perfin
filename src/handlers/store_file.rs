use std::{
    ffi::OsStr,
    fs,
    io::{self, Cursor, Read, Seek},
    path::Path,
};

use axum::{
    extract::{ContentLengthLimit, Multipart},
    response::IntoResponse,
};
use rand::prelude::random;
use serde::Serialize;
use tracing::{debug, error};

use crate::olconnect::template::OlTemplate;

#[derive(Serialize)]
struct HelloTemplate {
    name: String,
}

const SAVE_FILE_BASE_PATH: &str = "./artefacts";

fn save_as_zip(contents: impl Read + Seek, base: &String) -> i32 {
    let zipper = zip::ZipArchive::new(contents);
    if let Ok(mut archive) = zipper {
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };
            let qualified = format!("{}/{}", base, outpath.display()).replace("\\", "/");
            let outpath = Path::new(&qualified);

            {
                let comment = file.comment();
                if !comment.is_empty() {
                    println!("File {} comment: {}", i, comment);
                }
            }

            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
                // Get and Set permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;

                    if let Some(mode) = file.unix_mode() {
                        fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                    }
                }
            }
        }
    }

    0
}

pub async fn store_file(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            1024 * 1024 * 20 //20M
        },
    >,
) -> impl IntoResponse {
    if let Some(file) = multipart.next_field().await.unwrap() {
        //file type
        let content_type = file.content_type().unwrap().to_string();
        let filename = file.file_name().unwrap().to_string();
        let osfilename = Path::new(&filename);
        debug!(
            "received file name {} [{}]",
            osfilename.display(),
            content_type
        );
        let file_extension = osfilename
            .extension()
            .and_then(OsStr::to_str)
            .or_else(|| Some(".unknown"))
            .unwrap();

        let rnd = format!("{}", (random::<f32>() * 1000000000 as f32) as i32);
        // Focus name that is finally saved on the server

        //document content
        //
        if file_extension.starts_with("OL-") {
            let basename = osfilename
                .file_stem()
                .and_then(OsStr::to_str)
                .or_else(|| Some(&rnd.as_str()))
                .unwrap();
            let folder: String = file_extension
                .chars()
                .skip(3)
                .take(file_extension.len() - 3)
                .collect();
            let text = file.bytes().await.unwrap();
            let save_folder_name = format!("{}/{}/{}", SAVE_FILE_BASE_PATH, folder, basename);
            debug!(
                "Save {} {} with size {} bytes ({})",
                folder,
                basename,
                text.len(),
                save_folder_name
            );
            let rb = Cursor::new(text);
            let save_folder_name = format!("{}/{}/{}", SAVE_FILE_BASE_PATH, folder, basename);
            fs::create_dir_all(&save_folder_name).unwrap();
            save_as_zip(rb, &save_folder_name);

            if "OL-template".eq(file_extension) {
                let template_name = basename.to_string();
                let ol_template = OlTemplate::new(&template_name);
                debug!(
                    "Trying from {}",
                    format!("./artefacts/template/{}/index.xml", template_name)
                );
                match ol_template.read_xml() {
                    Ok(_) => return format!("{}/{}", folder, template_name,),
                    Err(_) => {
                        let _ = fs::remove_dir_all(&save_folder_name);
                        return "Error: Invalid template file".to_string();
                    }
                }
            } else {
                return format!("{}/{}", folder, basename,);
            }
        } else {
            let save_folder_name = format!("{}/{}.{}", SAVE_FILE_BASE_PATH, rnd, file_extension);
            let data = file.bytes().await.unwrap();
            // Auxiliary log
            debug!("save as:{}", &save_folder_name);

            // Save the uploaded file
            match tokio::fs::write(&save_folder_name, &data)
                .await
                .map_err(|err| err.to_string())
            {
                Ok(_) => return format!("{}.{}", rnd, file_extension),
                Err(e) => return e,
            }
        }
    }

    // Normal situation, can't get here
    error!("{}", "There is no upload file or file format is wrong");

    // When the uploaded file type is wrong, the following redirect will fail (feeling the bug of Axum)
    return "No file".to_string();
}
