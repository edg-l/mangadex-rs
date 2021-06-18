# mangadex-rs

[![Rust](https://github.com/edg-l/mangadex-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/edg-l/mangadex-rs/actions/workflows/rust.yml)
[![crates.io](http://meritbadge.herokuapp.com/mangadex)](https://crates.io/crates/mangadex)
[![License](https://img.shields.io/github/license/edg-l/mangadex-rs)](https://github.com/edg-l/mangadex-rs/blob/master/LICENSE)

> Early work in progress.

Unofficial asynchronous mangadex API wrapper for rust

https://rust-lang.github.io/api-guidelines/checklist.html

The api reference is available in the [default](https://api.mangadex.org/docs.html) format, or in
[swagger](https://api.mangadex.org/swagger.html) format. The currently targeted api (`api.yaml`) is
tracked in the repository to facilitate easier updates across versions.

## TODO

- Missing endpoints
  - Manga
    - `GET manga/{id}/aggregate`
  - Upload
    - `POST /cover/{mangaId}`
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
