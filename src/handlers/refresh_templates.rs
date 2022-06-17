use std::sync::Arc;

use axum::{extract::Extension, response::IntoResponse};
use tracing::info;

use crate::perfin_app::PerfinApp;

pub async fn refresh_templates(Extension(app): Extension<Arc<PerfinApp>>) -> impl IntoResponse {
    let app = app.clone();
    let mut template_renderer = app.use_template_renderer();

    let response_text = match template_renderer.refresh_templates() {
        Ok(_) => return "Ok".to_string(),
        Err(err) => format!("{}", err),
    };
    info!("Here is the result of the refresh {}", response_text);
    return response_text;
}
