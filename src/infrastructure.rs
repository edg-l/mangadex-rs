use crate::{errors::Errors, Client, Result};

impl Client {
    pub async fn ping(&self) -> Result<()> {
        let endpoint = self.base_url.join("/ping")?;

        let res = self.http.get(endpoint).send().await?;
        if res.text().await? == "pong" {
            Ok(())
        } else {
            Err(Errors::PingError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ping_server() {
        let client = Client::new().unwrap();
        client.ping().await.unwrap();
    }
}
