use reqwest::{header::USER_AGENT, Client};

pub async fn request_auth(url: &str, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(Client::new()
        .get(url)
        .header(USER_AGENT, "megatank58")
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await?)
}

pub async fn request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(Client::new()
        .get(url)
        .header(USER_AGENT, "megatank58")
        .send()
        .await?
        .text()
        .await?)
}
