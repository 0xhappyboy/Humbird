use crate::protocol::http::HttpRequestProcess;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref ROUTER_TABLE: Mutex<HashMap<String, HttpRequestProcess>> = {
        let map = HashMap::new();
        Mutex::new(map)
    };
}

/// macro for registering web routes,work before starting humbird service
///
/// Example
/// ```rust
/// // register plugin
/// fn router_function(req: Request, mut res: Response) -> Response {
/// // ......
/// res
/// }
/// router!("/path" => router_function);
/// // run humbird server
/// run!();
/// ```
#[macro_export]
macro_rules! router {
    ($($path:expr => $process:expr),*) => {
        $(
            $crate::core::plugins::ROUTER_TABLE.lock().unwrap().insert($path.to_string(), $process);
        )*;
    };
}
