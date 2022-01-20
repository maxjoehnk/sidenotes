use crate::parser::{parse_soap_response, CalendarItem, FindItemResponseMessage};
use chrono::{DateTime, Duration, Local};
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::str::FromStr;

pub(crate) type TZ = Local;

mod parser;

#[derive(Debug)]
pub enum ExchangeVersion {
    Exchange2016,
}

impl FromStr for ExchangeVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Exchange2016" => Ok(Self::Exchange2016),
            _ => Err(()),
        }
    }
}

impl Display for ExchangeVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct EwsClient {
    url: String,
    username: String,
    password: String,
}

impl EwsClient {
    pub fn new(
        url: impl Into<String>,
        _: ExchangeVersion,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            username: username.into(),
            password: password.into(),
        }
    }

    pub async fn find_items(&self) -> anyhow::Result<Vec<CalendarItem>> {
        let start = TZ::now();
        let end = start.date().add(Duration::days(1)).and_hms(0, 0, 0);
        tracing::debug!("Fetching Calendar items between {} and {}", start, end);
        let mut response = surf::post(&self.url)
            .body(self.get_find_items_request(start, end))
            .header("Authorization", self.get_authorization_header())
            .send()
            .await
            .map_err(|err| anyhow::anyhow!("Request failed {:?}", err))?;

        anyhow::ensure!(response.status().is_success());

        let body = response
            .body_string()
            .await
            .map_err(|err| anyhow::anyhow!("Response reading failed {:?}", err))?;

        let deserialized = parse_soap_response::<FindItemResponseMessage>(body)?;

        Ok(deserialized.body.response.root_folder)
    }

    fn get_find_items_request(&self, start: DateTime<TZ>, end: DateTime<TZ>) -> String {
        format!(
            r#"<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages" xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
    <s:Header>
        <t:RequestServerVersion Version="Exchange2016"></t:RequestServerVersion>
    </s:Header>
    <s:Body>
        <m:FindItem Traversal="Shallow">
            <m:ItemShape>
                <t:BaseShape>IdOnly</t:BaseShape>
                <t:AdditionalProperties>
                    <t:FieldURI FieldURI="item:Subject"></t:FieldURI>
                    <t:FieldURI FieldURI="calendar:Start"></t:FieldURI>
                    <t:FieldURI FieldURI="calendar:End"></t:FieldURI>
                    <t:FieldURI FieldURI="calendar:When"></t:FieldURI>
                    <t:FieldURI FieldURI="calendar:Organizer"></t:FieldURI>
                    <t:FieldURI FieldURI="calendar:Location"></t:FieldURI>
                </t:AdditionalProperties>
            </m:ItemShape>
            <m:CalendarView StartDate="{}" EndDate="{}"></m:CalendarView>
            <m:ParentFolderIds>
                <t:DistinguishedFolderId Id="calendar">
                    <t:Mailbox>
                        <t:EmailAddress>{}</t:EmailAddress>
                    </t:Mailbox>
                </t:DistinguishedFolderId>
            </m:ParentFolderIds>
        </m:FindItem>
    </s:Body>
</s:Envelope>"#,
            start.to_rfc3339(),
            end.to_rfc3339(),
            self.username
        )
    }

    fn get_authorization_header(&self) -> String {
        let decoded = format!("{}:{}", self.username, self.password);
        let encoded = base64::encode(decoded);

        format!("Basic {}", encoded)
    }
}
