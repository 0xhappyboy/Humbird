mod cli;
use crate::cli::cli::Cli;
use clap::Parser;
use humbird_lib::server::server::*;
use tokio::join;

#[tokio::main]
async fn main() {
    //let cli = Cli::parse();
    join!(Server::run());
}
