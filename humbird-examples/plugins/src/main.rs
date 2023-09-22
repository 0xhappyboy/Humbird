use humbird::{
    core::server::Server,
    protocol::http::{Request, Response},
    register_router_plugin,
};

fn test(req: Request, mut res: Response) -> Response {
    res.set_body("rewrite");
    res
}

fn main() {
    // register router
    register_router_plugin!(
        "/"=>test
    );
    Server::config_run(
        humbird::core::server::NetModel::EventPoll,
        "config-template.toml",
    );
}
