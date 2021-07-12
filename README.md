# mangadex-rs

[![Version](https://img.shields.io/crates/v/mangadex)](https://crates.io/crates/mangadex)
[![Downloads](https://img.shields.io/crates/d/mangadex)](https://crates.io/crates/mangadex)
[![License](https://img.shields.io/crates/l/mangadex)](https://crates.io/crates/mangadex)
![Rust](https://github.com/edg-l/mangadex-rs/workflows/Rust/badge.svg)
[![Docs](https://docs.rs/mangadex/badge.svg)](https://docs.rs/mangadex)

Unofficial asynchronous mangadex API wrapper for rust.

https://rust-lang.github.io/api-guidelines/checklist.html

The api reference is available in the [default](https://api.mangadex.org/docs.html) format, or in
[swagger](https://api.mangadex.org/swagger.html) format. The currently targeted api (`api.yaml`) is
tracked in the repository to facilitate easier updates across versions.

## TODO

- Upload
- Rate limiting
- Includes

## Tests

Tests that require making calls to the api while authed are ignored by default, to run them you need to setup a .env file likes this:

```bash
TEST_MANGADEX_USERNAME="username"
TEST_MANGADEX_PASSWORD="password"
```

And run it with:

`cargo test -- --ignored`
