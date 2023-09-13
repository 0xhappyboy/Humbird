use crate::protocol::http::HttpRequestProcess;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref ROUTER_TABLE: Mutex<HashMap<String, HttpRequestProcess>> = {
        let mut map = HashMap::new();
        Mutex::new(map)
    };
}

/// macro for registering web routes,work before starting humbird service
///
/// Example
/// ```rust
/// // register plugin
/// register_router_plugin!("/path".to_string() => function_name);
/// // run humbird server
/// run!();
/// ```
#[macro_export]
macro_rules! register_router_plugin {
    ($($path:expr => $process:expr),*) => {
        $(
            $crate::plugins::web::ROUTER_TABLE.lock().unwrap().insert($path, $process);
        )*;
    };
}
