use axum::{extract, response::IntoResponse};

use serde::Deserialize;

use crate::{
    olconnect::template::{OlTemplate, Result},
    util::ContentType,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceContent {
    template: String,
    resource_type: String,
    resource_id: String,
}

impl ResourceContent {
    fn read_resource_contents(&self) -> Result<ContentType> {
        let ol_template = OlTemplate::new(&self.template);
        ol_template.read_resource_contents(&self.resource_type, &self.resource_id)
    }
}

pub async fn resource_content(
    extract::Json(payload): extract::Json<ResourceContent>,
) -> impl IntoResponse {
    match payload.read_resource_contents() {
        Ok(content) => content,
        Err(e) => ContentType::Text(format!("{}", e)),
    }
}
