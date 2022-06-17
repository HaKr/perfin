use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};
use handlebars::Context;
use serde::Serialize;
use tracing::info;

use crate::handlebars_engine::HandlebarsEngine;

use super::handlebars_response::HandlebarsResponse;

#[derive(Serialize)]
struct HelloTemplate {
    name: String,
}

pub async fn greet(
    Extension(render_engine): Extension<Arc<HandlebarsEngine>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    info!("Handle a greet request for {}", name);
    use std::{thread, time};
    thread::sleep(time::Duration::from_secs(16));
    return HandlebarsResponse {
        engine: render_engine.clone(),
        template_name: "greeter",
        template_data: Context::wraps(&HelloTemplate { name }).unwrap(),
    };
}
