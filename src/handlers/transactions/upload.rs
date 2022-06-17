#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    borrow::Borrow, collections::HashMap, ffi::OsStr, fmt::Display, io::BufReader, ops::Deref,
    path::Path, sync::Arc,
};

use axum::{
    body::Bytes,
    extract::{multipart::Field, ContentLengthLimit, Extension, Multipart},
    response::{Html, IntoResponse},
    Json,
};

use handlebars::Context;
use rand::random;
use serde::Serialize;
use tracing::{debug, error, info};

use crate::{
    html_template_renderer::HtmlTemplateRenderer,
    ing::{DescriptionProperties, IngImporter, IngTransaction},
    AccountHibernate, AccountsRepository, BankTransaction, CostCentersRepository, Error, PerfinApp,
};

const SAVE_FILE_BASE_PATH: &str = "./data/storage/upload";

#[derive(PartialEq)]
enum BankImportType {
    Unrecognised,
    IngCsv,
}

enum ContentType {
    Csv,
    Json,
}

struct FileAttachment {
    filename: String,
    content_type: ContentType,
    contents: Bytes,
}

struct FormData {
    import_type: BankImportType,
    attachment: FileAttachment,
}

#[derive(Serialize)]
struct Indices {
    imported: i64,
    assigned: i64,
}

#[derive(Serialize)]
struct UploadContext {
    accounts: Vec<AccountHibernate>,
    indices: Indices,
    failures: Option<Vec<String>>,
    imported: Option<HashMap<String, Vec<BankTransaction>>>,
    assigned: Option<HashMap<String, Vec<BankTransaction>>>,
}

pub async fn upload(
    Extension(app): Extension<Arc<PerfinApp>>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            1024 * 1024 * 20 //20M
        },
    >,
) -> impl IntoResponse {
    info!("Upload request");
    if let Some(form_data) = FormData::from_mime(&mut multipart).await {
        let app = app.clone();
        let mut template_renderer = app.use_template_renderer();
        let /*mut*/ ledger = app.use_ledger();

        let data: &[u8] = &form_data.attachment.contents.as_ref();
        let mut importer = IngImporter::from_reader(data);

        let mut failures = vec![];
        let mut imported: HashMap<String, Vec<BankTransaction>> = HashMap::new();
        let mut assigned: HashMap<String, Vec<BankTransaction>> = HashMap::new();

        info!("\tstart parsing transactions");

        let ledger_ref = ledger.deref();
        for result in importer.transactions(ledger_ref, ledger_ref, ledger_ref) {
            match result {
                Ok(bank_transaction) => {
                    let transaction_account_code = bank_transaction.account_code.clone();
                    if let Some(account_code) = transaction_account_code {
                        let account = ledger
                            .find_account_by_reference(account_code.as_str())
                            .unwrap();
                        let key = format!("{} - {}", account_code, account.description);
                        let entry = assigned.entry(key).or_insert_with(Vec::default);
                        (*entry).push(bank_transaction);
                    } else {
                        let optional_relation_name = bank_transaction.relation_name.clone();
                        let key = match optional_relation_name {
                            Some(key) => key,
                            None => String::from("[Unknown]"),
                        };
                        let entry = imported.entry(key).or_insert_with(Vec::default);
                        (*entry).push(bank_transaction);
                    }
                }
                Err(e) => failures.push(e),
            }
        }

        info!("\tparsed; rendering");

        let html_text = template_renderer
            .render(
                "upload_result",
                &UploadContext {
                    failures: if failures.len() > 0 {
                        Some(failures.iter().map(|e| format!("{:?}", e)).collect())
                    } else {
                        None
                    },
                    imported: if imported.len() > 0 {
                        Some(imported)
                    } else {
                        None
                    },
                    assigned: if assigned.len() > 0 {
                        Some(assigned)
                    } else {
                        None
                    },
                    accounts: ledger.accounts_for_hibernate(),
                    indices: Indices {
                        imported: 0,
                        assigned: 0,
                    },
                },
            )
            .unwrap();

        info!("\trendered {} bytes", html_text.len());

        Html(html_text)
    } else {
        Html("Error: FormData invalid".to_owned())
    }
}

impl From<&str> for BankImportType {
    fn from(str: &str) -> Self {
        if "ing".eq_ignore_ascii_case(str) {
            Self::IngCsv
        } else {
            Self::Unrecognised
        }
    }
}

impl Display for BankImportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BankImportType::Unrecognised => f.write_str("unrecognised"),
            BankImportType::IngCsv => f.write_str("ing"),
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Csv => f.write_str("text/csv"),
            ContentType::Json => f.write_str("application/json"),
        }
    }
}

impl ContentType {
    pub fn from_extension(ext: &str) -> Option<Self> {
        if "csv".eq(ext) {
            Some(Self::Csv)
        } else if "json".eq_ignore_ascii_case(ext) {
            Some(Self::Json)
        } else {
            None
        }
    }

    pub fn from_header(hdr: &str) -> Option<Self> {
        if "text/csv".eq_ignore_ascii_case(hdr) {
            Some(Self::Csv)
        } else if "application/json".eq_ignore_ascii_case(hdr) {
            Some(Self::Json)
        } else {
            None
        }
    }
}

impl FileAttachment {
    fn get_file_name_and_content_type<'f>(field: &'f Field<'f>) -> Option<(String, ContentType)> {
        if let Some(field_filename) = field.file_name() {
            let osfilename = Path::new(&field_filename);
            let ext = osfilename
                .extension()
                .and_then(OsStr::to_str)
                .or_else(|| Some(".unknown"))
                .unwrap();
            if let Some(content_type) = ContentType::from_extension(ext) {
                return Some((format!("{}", field_filename), content_type));
            }
        }

        None
    }

    pub async fn from_mime<'f>(field: Field<'f>) -> Option<Self> {
        let data = Self::get_file_name_and_content_type(&field);
        if let Some((filename, content_type)) = data {
            if let Ok(contents) = field.bytes().await {
                return Some(Self {
                    content_type,
                    filename,
                    contents: contents.to_owned(),
                });
            }
        }

        None
    }
}

impl FormData {
    pub async fn from_mime(multipart: &mut Multipart) -> Option<Self> {
        let mut import_type: Option<BankImportType> = None;
        let mut attachment: Option<FileAttachment> = None;
        let mut has_fields = true;
        while has_fields && (import_type.is_none() || attachment.is_none()) {
            let next_field_result = multipart.next_field().await;
            match next_field_result {
                Ok(next_field_option) => match next_field_option {
                    Some(field) => {
                        if let Some(name) = field.name() {
                            if "format".eq_ignore_ascii_case(name) {
                                let data = field.text().await;
                                if let Ok(import_type_code) = data {
                                    let candidate = BankImportType::from(import_type_code.as_str());
                                    if candidate != BankImportType::Unrecognised {
                                        import_type = Some(candidate);
                                    }
                                }
                            } else if "transactions_file".eq_ignore_ascii_case(name) {
                                attachment = FileAttachment::from_mime(field).await
                            }
                        }
                    }
                    None => has_fields = false,
                },
                Err(e) => {
                    error!("Multipart field error {}", e);
                }
            }
        }

        if let Some(import_type) = import_type {
            if let Some(attachment) = attachment {
                return Some(Self {
                    attachment,
                    import_type,
                });
            }
        }

        None
    }
}
