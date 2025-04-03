use tokio::time::{Duration, sleep};

pub async fn db_stuff() {
    let duration = Duration::from_millis(500);
    sleep(duration).await;
}
