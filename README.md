# mangadex-rs
[![Rust](https://github.com/edg-l/mangadex-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/edg-l/mangadex-rs/actions/workflows/rust.yml)
[![crates.io](http://meritbadge.herokuapp.com/mangadex)](https://crates.io/crates/mangadex)
[![License](https://img.shields.io/github/license/edg-l/mangadex)](https://github.com/edg-l/mangadex/blob/master/LICENSE)

> Early work in progress.

Unofficial asynchronous mangadex API wrapper for rust

https://rust-lang.github.io/api-guidelines/checklist.html

https://api.mangadex.org/docs.html

## Tests
Tests that require making calls to the api are ignored by default, to run them you need to setup a .env file likes this:

```bash
TEST_MANGADEX_USERNAME="username"
TEST_MANGADEX_PASSWORD="password"
```

And run it with:

`cargo test -- --ignored`
