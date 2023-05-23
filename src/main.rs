mod common;
mod infrastructure;
mod routes;
mod server;
mod todo;

#[tokio::main]
async fn main() {
    server::create_server().await;
}
