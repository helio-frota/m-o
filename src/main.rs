use actix_web::{App, HttpRequest, HttpServer, web};
use opentelemetry_instrumentation_actix_web::RequestMetrics;

// use std::sync::Once;

mod otel;
use crate::otel::init_metrics;
mod db_layer;
mod service_layer;
use crate::service_layer::service_stuff;

// use actix_web_opentelemetry::{PrometheusMetricsHandler, RequestMetrics};

async fn hello(_req: HttpRequest) -> &'static str {
    service_stuff().await;
    "Hello"
}

// static INIT: Once = Once::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // INIT.call_once(|| init_metrics("m-o"));
    init_metrics("m-o");

    HttpServer::new(|| {
        App::new()
            .wrap(RequestMetrics::default())
            // .route(
            //     "/metrics",
            //     web::get().to(PrometheusMetricsHandler::new(registry.clone())),
            // )
            .service(web::resource("/").to(hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
