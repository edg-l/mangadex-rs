//! Server status

use crate::{Client, Result};

/// Ping the server
///
/// Call to `GET /ping`
pub struct Ping;

impl Ping {
    pub async fn send(&self, client: &Client) -> Result<()> {
        // `GET /ping` does not return JSON, because
        // it's a special snow-flake. So we get this...
        client.ping().await
    }
}
