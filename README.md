# mangadex-rs
Early work in progress.

An unofficial asynchronous mangadex API wrapper for rust

https://rust-lang.github.io/api-guidelines/checklist.html

https://api.mangadex.org/docs.html

## Tests
Tests that require making calls to the api are ignored by default, to run them you need to setup a .env file likes this:

TEST_MANGADEX_USERNAME="username"
TEST_MANGADEX_PASSWORD="password"

And run it with:

`cargo test -- --ignored`
