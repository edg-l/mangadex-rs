# mangadex-rs

[![Rust](https://github.com/edg-l/mangadex-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/edg-l/mangadex-rs/actions/workflows/rust.yml)
[![crates.io](http://meritbadge.herokuapp.com/mangadex)](https://crates.io/crates/mangadex)
[![License](https://img.shields.io/github/license/edg-l/mangadex-rs)](https://github.com/edg-l/mangadex-rs/blob/master/LICENSE)

> Early work in progress.

Unofficial asynchronous mangadex API wrapper for rust

https://rust-lang.github.io/api-guidelines/checklist.html

Api reference: https://api.mangadex.org/docs.html

## TODO

- [ ] Missing endpoints
  - [ ] Manga
    - [ ] `GET manga/{id}/aggregate`
  - [ ] Upload
    - [ ] `POST /cover/{mangaId}`
  - [ ] User
  - [ ] Chapter
- [ ] Rate limiting
- [ ] Includes

## Tests

Tests that require making calls to the api while authed are ignored by default, to run them you need to setup a .env file likes this:

```bash
TEST_MANGADEX_USERNAME="username"
TEST_MANGADEX_PASSWORD="password"
```

And run it with:

`cargo test -- --ignored`
