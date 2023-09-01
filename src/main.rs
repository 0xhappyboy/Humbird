mod config;
mod server;

use server::server::*;
use tokio::join;

#[tokio::main]
async fn main() {
    join!(Server::run());
}
