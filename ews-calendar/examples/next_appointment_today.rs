use ews_calendar::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::var("EWS_URL")?;
    let username = std::env::var("EWS_USERNAME")?;
    let password = std::env::var("EWS_PASSWORD")?;

    let api = EwsClient::new(url, ExchangeVersion::Exchange2016, username, password);

    let items = api.find_items().await?;

    println!("{:#?}", items);

    Ok(())
}
