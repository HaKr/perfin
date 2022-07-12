use std::{net::SocketAddr, sync::Arc};

#[allow(unused_imports)]
use axum::{
    extract::Extension,
    routing::{get as GET, get_service, post as POST},
    Router,
};

use http::HeaderValue;
use hyper::StatusCode;
use perfin::{handlers, HtmlTemplateRenderer, Ledger, PerfinApp};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

use tower::ServiceBuilder;

use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=info,perfin=debug")
    }
    tracing_subscriber::fmt::init();

    // TODO retrieve infrom from Session
    let ledger = Ledger::load("cb09add43080499a90e7479543e750a9", 2022).expect("load ledger");

    let content_security_policy = SetResponseHeaderLayer::if_not_present(
        http::header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "script-src 'self' 'unsafe-inline'; script-src-elem 'self' http://192.168.37.128:300; img-src 'self' data:",
        ),
    );

    match HtmlTemplateRenderer::new() {
        Ok(template_renderer) => {
            let app = Arc::new(PerfinApp::new(template_renderer, ledger));

            let error_responder = |error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            };

            let app_server = Router::new()
                .nest(
                    "/scripts",
                    get_service(ServeDir::new("www/scripts")).handle_error(error_responder),
                )
                .nest(
                    "/styles",
                    get_service(ServeDir::new("www/styles")).handle_error(error_responder),
                )
                .route(
                    "/favicon.ico",
                    get_service(ServeFile::new("www/favicon.ico")).handle_error(error_responder),
                )
                .route("/", GET(handlers::index))
                // .route("/store/store_file", POST(store_file))
                // .route("/store/resource_content", POST(resource_content))
                .route("/admin/refresh_templates", GET(handlers::refresh_templates))
                .route("/transactions/import", GET(handlers::transactions::import))
                .route(
                    "/upload/bank_transactions",
                    POST(handlers::transactions::upload),
                )
                // .route("/greet/:name", GET(greet))
                // .route("/template/:template/image/:image_id", GET(image))
                .layer(
                    ServiceBuilder::new()
                        .layer(TraceLayer::new_for_http())
                        .layer(Extension(app))
                        .layer(content_security_policy),
                );
            let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

            info!("listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(app_server.into_make_service())
                .await
                .unwrap();
        }
        Err(template_error) => error!("Handlebars error {}", template_error),
    }
}
