//! [![Rust](https://github.com/edg-l/mangadex-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/edg-l/mangadex-rs/actions/workflows/rust.yml)
//! [![crates.io](http://meritbadge.herokuapp.com/mangadex)](https://crates.io/crates/mangadex)
//! [![License](https://img.shields.io/github/license/edg-l/mangadex-rs)](https://github.com/edg-l/mangadex-rs/blob/master/LICENSE)
//!
//! Unofficial asynchronous mangadex API wrapper for rust
//!
//! Example login:
//!
//! ```rust,no_run
//! use mangadex::Client;
//! use mangadex::api::{auth, manga};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = Client::default();
//!     client.login("username", "password").await.unwrap();
//! }
//! ```
//!
//! How to get some mangas (you don't need to be logged in for most actions):
//! 
//! ```rust,no_run
//! use mangadex::api::manga::*;
//! use mangadex::schema::manga::*;
//! use mangadex::schema::LanguageCode;
//! use mangadex::Client;
//! 
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Client::default();
//! 
//!     let list_manga = ListMangaBuilder::default()
//!         .add_status(MangaStatus::Ongoing)
//!         .build()?;
//! 
//!     let mangas = list_manga.send(&client).await?;
//! 
//!     for manga_result in &mangas.results {
//!         if let Ok(manga) = &manga_result {
//!             let english_title = manga
//!                 .data
//!                 .attributes
//!                 .title
//!                 .get(&LanguageCode::English)
//!                 .unwrap();
//!             println!("Got manga {} with id: {:x}", english_title, manga.data.id);
//!         }
//!     }
//! 
//!     Ok(())
//! }
//! ```

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
