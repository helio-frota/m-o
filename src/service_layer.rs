use tokio::time::{sleep, Duration};

use crate::db_layer::db_stuff;
use crate::otel::basic_counter;

pub async fn service_stuff() {
    let counter = basic_counter(
        "service_stuff_calls",
        "Total calls of service_stuff function",
    );

    counter.add(1, &[]);
    let duration = Duration::from_millis(300);
    sleep(duration).await;
    db_stuff().await;
}
