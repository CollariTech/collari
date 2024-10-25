mod app;
mod api;
mod json;

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    app::Server::new()
        .run()
        .await
}