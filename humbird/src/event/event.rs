use serde::{Deserialize, Serialize};

/// global constants related to event
lazy_static! {
   /// event pool
   pub static ref EVENT_POOL: Mutex<Vec{}> = Mutex::new(String::from(DEFAULT_SERVER_LISTENING_PORT.to_string()));
}

#[derive(Serialize, Deserialize)]
struct Event {}

impl Event {}
