use crate::USER_AGENT;
use anyhow::bail;
use reqwest::{Client, IntoUrl};

fn client() -> anyhow::Result<Client> {
    let client = Client::builder().user_agent(USER_AGENT).build()?;

    Ok(client)
}

pub(crate) async fn make_json_request<U: IntoUrl>(
    uri: U,
) -> anyhow::Result<Option<serde_json::Value>> {
    let client = client()?;
    let response = client
        .get(uri)
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?;

    if response.status().as_u16() == 404 {
        return Ok(None);
    }

    if !response.status().is_success() {
        bail!(
            "request failed: {} - {}",
            response.status().as_u16(),
            response.text().await?
        );
    }

    let json = response.json::<serde_json::Value>().await?;
    Ok(Some(json))
}
