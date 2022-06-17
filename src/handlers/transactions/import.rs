use std::sync::Arc;

use axum::{extract::Extension, response::IntoResponse};

use serde::Serialize;

use crate::{handlers::render_html_template, AccountHibernate, BankFormats, PerfinApp};

#[derive(Serialize)]
struct ImportContext {
    formats: BankFormats,
    accounts: Vec<AccountHibernate>,
}

pub async fn import(
    Extension(app): Extension<Arc<PerfinApp>>,
    // Query(selected): Query<SelectedTemplate>,
) -> impl IntoResponse {
    let formats = BankFormats::from_fixture().unwrap_or_else(|_| BankFormats::default());
    render_html_template(app, "statements_import", |ledger| ImportContext {
        formats,
        accounts: ledger.accounts_for_hibernate(),
    })
}
