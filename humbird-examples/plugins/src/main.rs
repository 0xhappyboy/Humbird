use humbird::{
    core::server::Server,
    protocol::http::{Request, Response},
    http_router,
};

fn test(req: Request, mut res: Response) -> Response {
    res.set_body("rewrite");
    res
}

fn main() {
    // register router
    http_router!(
        "/"=>test
    );
    Server::config_run("config-template.toml");
}
