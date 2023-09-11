use crate::protocol::http::r#type::HttpRequestProcess;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref ROUTER_TABLE: Mutex<HashMap<String, HttpRequestProcess>> = {
        let mut map = HashMap::new();
        Mutex::new(map)
    };
}

#[macro_export]
macro_rules! register_router_plugin {
    ($($path:expr => $process:expr),*) => {
        $(
            $crate::plugins::web::ROUTER_TABLE.lock().unwrap().insert($path, $process);
        )*;
    };
}
