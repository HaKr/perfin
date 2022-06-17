use std::sync::Arc;

use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
};
use handlebars::Context;
use serde::{Deserialize, Serialize};

use crate::handlebars_engine::HandlebarsEngine;

use super::handlebars_response::HandlebarsResponse;

#[derive(Deserialize, Debug)]
pub struct Departure {
    lunian: Option<String>,
    duration: Option<u8>,
}

#[derive(Serialize)]
struct DepartureData {
    lunian: String,
    duration: u8,
}

const LUNIAN: &str = "Lunian";

pub async fn byebye(
    Extension(render_engine): Extension<Arc<HandlebarsEngine>>,
    Query(departure): Query<Departure>,
) -> impl IntoResponse {
    return HandlebarsResponse {
        engine: render_engine.clone(),
        template_name: "farewell",
        template_data: Context::wraps(&DepartureData {
            lunian: String::from(departure.lunian.unwrap_or(LUNIAN.to_string())),
            duration: departure.duration.unwrap_or(2),
        })
        .unwrap(),
    };
}
