use opentelemetry::KeyValue;
use tokio::time::{Duration, sleep};

use crate::db_layer::db_stuff;
use crate::otel::basic_counter;

pub async fn service_stuff() {
    let counter = basic_counter(
        "service_stuff_calls",
        "Total calls of service_stuff function",
    );

    counter.add(
        1,
        &[
            KeyValue::new("service", "service_stuff"),
            KeyValue::new("status", "success"),
        ],
    );

    let duration = Duration::from_millis(200);
    sleep(duration).await;
    db_stuff().await;
}
