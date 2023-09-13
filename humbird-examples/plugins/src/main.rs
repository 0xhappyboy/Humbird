use humbird::protocol::http::Request;
use humbird::protocol::http::Response;
use humbird::register_router_plugin;
use humbird::run;
// plugin function
fn test(mut request: Request, mut response: Response) -> Response {
    // here you can modify a and b
    return response;
}

fn main() {
    // register plugin
    register_router_plugin!("/test".to_string() => test);
    // run humbird server
    run!();
}
