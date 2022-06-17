use axum::{
    body::{self, boxed, BoxBody},
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use gotham::mime;
use hyper::header;

use std::{
    fs::File,
    io::{BufReader, Read},
};
pub enum ContentType {
    Html(File),
    Css(File),
    Plain(File),
    Js(File),
    Json(File),
    Jpeg(File),
    Text(String),
}

fn body_from_file(contents: &File) -> BoxBody {
    let mut buf: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(contents);
    reader.read_to_end(&mut buf).unwrap();
    boxed(body::Full::from(buf))
}

fn body_from_string(contents: String) -> BoxBody {
    boxed(body::Full::from(contents))
}

/*
            ContentType::Html(html) => (mime::TEXT_HTML.as_ref(), body_from_file(&html)),
            ContentType::Css(css) => (mime::TEXT_CSS.as_ref(), body_from_file(&css)),
            ContentType::Plain(plain) => (mime::TEXT_PLAIN.as_ref(), body_from_file((&plain)),
            ContentType::Js(js) => (mime::APPLICATION_JAVASCRIPT.as_ref(), body_from_file(&js)),
            ContentType::Json(json) => (mime::APPLICATION_JSON.as_ref(), body_from_file(&json)),
            ContentType::Jpeg(jpeg) => (mime::IMAGE_JPEG.as_ref(), body_from_file(&jpeg)),
            ContentType::Text(_) => todo!(),

*/

impl IntoResponse for ContentType {
    fn into_response(self) -> Response {
        let (content_type, contents) = match self {
            ContentType::Html(html) => (mime::TEXT_HTML.as_ref(), body_from_file(&html)),
            ContentType::Css(css) => (mime::TEXT_CSS.as_ref(), body_from_file(&css)),
            ContentType::Plain(_) => todo!(),
            ContentType::Js(js) => (mime::APPLICATION_JAVASCRIPT.as_ref(), body_from_file(&js)),
            ContentType::Json(json) => (mime::APPLICATION_JSON.as_ref(), body_from_file(&json)),
            ContentType::Jpeg(jpeg) => (mime::IMAGE_JPEG.as_ref(), body_from_file(&jpeg)),
            ContentType::Text(txt) => (mime::TEXT_PLAIN.as_ref(), body_from_string(txt)),
        };
        let mut res = Response::new(contents);
        res.headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
        res
    }
}
