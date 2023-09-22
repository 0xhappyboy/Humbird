use serde::{Deserialize, Serialize};

/// signal point
#[macro_export]
macro_rules! sig_point {
    ($task:expr) => {
        task::spawn($task);
    };
}

#[derive(Serialize, Deserialize)]
struct Signal {}

impl Signal {}
