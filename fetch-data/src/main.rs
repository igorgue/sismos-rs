use tokio;

use sismos::fetch_data::fetch_data;

#[tokio::main]
async fn main() {
    fetch_data().await;
}
