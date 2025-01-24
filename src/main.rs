use actix_web::{web, App, HttpRequest, HttpServer};

use tracing::{info, instrument};

use std::sync::Once;

mod otel;
use crate::otel::init_otel_traces;
mod db_layer;
mod service_layer;
use crate::service_layer::service_stuff;

#[instrument(skip_all)]
async fn hello(_req: HttpRequest) -> &'static str {
    service_stuff().await;
    "Hello"
}

static INIT: Once = Once::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    INIT.call_once(|| init_otel_traces("m-o"));

    info!("starting...");

    HttpServer::new(|| App::new().service(web::resource("/").to(hello)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
