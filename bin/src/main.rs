mod cli;
use crate::cli::cli::Cli;
use clap::Parser;
use humbird::server::server::*;
use tokio::join;

#[tokio::main]
async fn main() {
    //let cli = Cli::parse();
    join!(Server::run());
}
