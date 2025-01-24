use tokio::time::{sleep, Duration};

pub async fn db_stuff() {
    let duration = Duration::from_millis(1500);
    sleep(duration).await;
}
