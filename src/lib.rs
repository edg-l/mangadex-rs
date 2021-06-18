#[macro_use]
mod client;

mod common;
mod errors;
// mod jwt;

pub mod api;
pub mod schema;

pub(crate) use common::*;

pub use client::Client;
pub use errors::{Errors, Result};

pub use reqwest;

#[cfg(test)]
mod tests {
    use ctor::ctor;

    #[ctor]
    fn init() {
        dotenv::dotenv().ok();
    }
}
