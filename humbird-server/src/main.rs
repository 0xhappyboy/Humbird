use clap::Parser;
use cli::cli::Cli;
use humbird::{
    config::config::load_config,
    run,
};

mod cli;

fn main() {
    // load config
    //load_config("/Users/max/GitProject/Humbird/humbird-server/src/config-template.toml".to_string());
    // cli
    //let cli = Cli::parse();
    run!();
}
