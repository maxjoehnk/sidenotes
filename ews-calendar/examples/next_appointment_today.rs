use ews_calendar::*;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let url = env!("EWS_URL");
    let username = env!("EWS_USERNAME");
    let password = env!("EWS_PASSWORD");

    let api = EwsClient::new(url, ExchangeVersion::Exchange2016, username, password);

    let items = api.find_items().await?;

    println!("{:#?}", items);

    Ok(())
}
