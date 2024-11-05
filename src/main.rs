use homework::{config, start_server};

#[tokio::main]
async fn main() {
    start_server(config::server::LISTEN_ADDR, config::db::DATABASE_URL).await;
}
