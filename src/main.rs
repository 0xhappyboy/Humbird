mod config;
mod net;
use net::server::*;
use tokio::join;

#[tokio::main]
async fn main() {
    join!(HttpServer::run());
}
