use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub(crate) fn is_known_placeholder_token(token: &str) -> bool {
    matches!(
        token.trim().to_ascii_lowercase().as_str(),
        "replace-with-strong-token" | "change-me" | "changeme" | "replace-me"
    )
}

#[path = "config/core.rs"]
mod core;
#[path = "config/listener.rs"]
mod listener;
#[path = "config/observability.rs"]
mod observability;
#[path = "config/performance.rs"]
mod performance;
#[path = "config/resilience.rs"]
mod resilience;
#[path = "config/secrets.rs"]
mod secrets;
#[path = "config/security.rs"]
mod security;
#[cfg(test)]
#[path = "config/tests.rs"]
mod tests;
#[path = "config/upstream.rs"]
mod upstream;

pub use self::{
    core::*, listener::*, observability::*, performance::*, resilience::*, secrets::*, security::*,
    upstream::*,
};
