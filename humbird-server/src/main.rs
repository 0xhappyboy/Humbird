use clap::Parser;
use cli::cli::Cli;
use humbird::{
    core::server::Server,
    protocol::http::{Request, Response},
    router,
};

mod cli;

fn test(req: Request, mut res: Response) -> Response {
    res.set_body("西校区偶尔欧");
    res
}

fn main() {
    // cli
    let cli = Cli::parse();
    router!(
        "/"=>test
    );
    // Server::config_run("config-template.toml");
    Server::run();
}
