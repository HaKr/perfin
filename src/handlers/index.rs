use std::sync::Arc;

#[allow(unused_imports)]
use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
};

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

use crate::{AccountHibernate, PerfinApp};

use super::render_html_template::render_html_template;

#[derive(Serialize)]
struct IndexContext {
    accounts: Vec<AccountHibernate>,
}

pub async fn index(
    Extension(app): Extension<Arc<PerfinApp>>,
    // Query(selected): Query<SelectedTemplate>,
) -> impl IntoResponse {
    render_html_template(app, "index", |ledger| IndexContext {
        accounts: ledger.accounts_for_hibernate(),
    })
}
