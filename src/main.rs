mod routes;
mod server;

#[tokio::main]
async fn main() {
    server::create_server().await;
}
