use tokio::time::{sleep, Duration};

use crate::db_layer::db_stuff;

pub async fn service_stuff() {
    let duration = Duration::from_millis(300);
    sleep(duration).await;
    db_stuff().await;
}
