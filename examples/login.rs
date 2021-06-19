use mangadex::Client;

use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mut client = Client::default();

    client
        .login(
            &env::var("MANGADEX_USERNAME").unwrap(),
            &env::var("MANGADEX_PASSWORD").unwrap(),
        )
        .await?;

    Ok(())
}
