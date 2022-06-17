use std::sync::{Arc, MutexGuard};

use axum::response::{Html, IntoResponse};
use serde::Serialize;

use crate::{Ledger, PerfinApp};

pub fn render_html_template<F, T>(
    app: Arc<PerfinApp>,
    template_name: &str,
    create_context: F,
) -> impl IntoResponse
where
    F: FnOnce(MutexGuard<Ledger>) -> T,
    T: Serialize,
{
    let app = app.clone();
    let mut template_renderer = app.use_template_renderer();
    let ledger = app.use_ledger();
    let context = create_context(ledger);

    let html_text = match template_renderer.render(template_name, &context) {
        Ok(html) => html,
        Err(error) => format!("Index could not be rendered: {}", error),
    };

    Html(html_text)
}
