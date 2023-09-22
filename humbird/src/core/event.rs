use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

/// global event pool
lazy_static! {
    pub static ref ROUTER_TABLE: Mutex<HashMap<String, crate::core::event::EventHandle>> = {
        let map = HashMap::new();
        Mutex::new(map)
    };
}

/// event handle
pub type EventHandle = fn(Event) -> Event;

/// event type
pub enum EventType {}

#[derive(Serialize, Deserialize)]
pub struct Event {}

impl Event {}
