mod cli;
mod config;
mod server;
use crate::cli::cli::Cli;
use clap::Parser;
use server::server::Server;
use tokio::join;

#[tokio::main]
async fn main() {
    //let cli = Cli::parse();
    join!(Server::run());
}
