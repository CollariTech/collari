mod app;
mod api;

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    app::Server::new()
        .run()
        .await
}