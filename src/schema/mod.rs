//! Data structures for API abstraction

pub mod at_home;
pub mod auth;
pub mod author;
pub mod chapter;
pub mod cover;
pub mod errors;
pub mod feed;
pub mod group;
pub mod legacy;
pub mod list;
pub mod manga;
pub mod user;

mod common;
pub use common::*;
