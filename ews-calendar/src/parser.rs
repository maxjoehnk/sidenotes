use crate::TZ;
use chrono::DateTime;
use derive_builder::Builder;
use std::str::FromStr;
use xml::reader::{Events, XmlEvent};
use xml::EventReader;

pub trait XmlParser: Sized + Clone {
    fn parse(reader: &mut Events<&[u8]>) -> anyhow::Result<Self>;
}

#[derive(Clone, Debug, Builder)]
pub struct SoapMessage<TBody> {
    pub header: SoapHeader,
    pub body: SoapBody<TBody>,
}

pub fn parse_soap_response<TBody: XmlParser>(body: String) -> anyhow::Result<SoapMessage<TBody>> {
    let parser = EventReader::from_str(&body);
    let mut parser = parser.into_iter();

    let mut soap_message_builder = SoapMessageBuilder::default();

    while let Some(e) = parser.next() {
        let event = e?;
        if let XmlEvent::StartElement { name, .. } = event {
            match name.local_name.as_str() {
                "Header" => {
                    let header = SoapHeader::parse(&mut parser)?;
                    soap_message_builder.header(header);
                }
                "Body" => {
                    let body = SoapBody::parse(&mut parser)?;
                    soap_message_builder.body(body);
                }
                _ => continue,
            }
        }
    }
    let soap_message = soap_message_builder.build()?;

    Ok(soap_message)
}

#[derive(Clone, Debug)]
pub struct SoapHeader {}

impl XmlParser for SoapHeader {
    fn parse(reader: &mut Events<&[u8]>) -> anyhow::Result<Self> {
        while let Some(e) = reader.next() {
            let event = e?;
            match event {
                XmlEvent::EndElement { name } if name.local_name == "Header" => {
                    return Ok(SoapHeader {})
                }
                _ => continue,
            }
        }
        unreachable!()
    }
}

#[derive(Clone, Debug, Builder)]
pub struct SoapBody<TBody> {
    pub response: TBody,
}

impl<TBody: XmlParser> XmlParser for SoapBody<TBody> {
    fn parse(reader: &mut Events<&[u8]>) -> anyhow::Result<Self> {
        let mut builder = SoapBodyBuilder::default();
        while let Some(e) = reader.next() {
            let event = e?;
            match event {
                XmlEvent::EndElement { name } if name.local_name == "Body" => break,
                XmlEvent::StartElement { name, .. }
                    if name.local_name == "FindItemResponseMessage" =>
                {
                    let response_message = TBody::parse(reader)?;
                    builder.response(response_message);
                }
                _ => continue,
            }
        }
        let body = builder.build()?;

        Ok(body)
    }
}

#[derive(Clone, Debug, Builder)]
pub struct FindItemResponseMessage {
    pub root_folder: Vec<CalendarItem>,
}

impl XmlParser for FindItemResponseMessage {
    fn parse(reader: &mut Events<&[u8]>) -> anyhow::Result<Self> {
        let mut builder = FindItemResponseMessageBuilder::default();
        while let Some(e) = reader.next() {
            let event = e?;
            match event {
                XmlEvent::EndElement { name } if name.local_name == "FindItemResponseMessage" => {
                    break
                }
                XmlEvent::StartElement { name, .. } if name.local_name == "Items" => {
                    let items = Vec::<CalendarItem>::parse(reader)?;
                    builder.root_folder(items);
                }
                _ => continue,
            }
        }
        let message = builder.build()?;

        Ok(message)
    }
}

#[derive(Debug, Clone, Builder)]
pub struct CalendarItem {
    pub id: String,
    pub subject: String,
    pub start: DateTime<TZ>,
    pub end: DateTime<TZ>,
    #[builder(default)]
    pub location: Option<String>,
    // TODO: parse organizer
    // pub organizer: Option<Mailbox>
}

impl XmlParser for Vec<CalendarItem> {
    fn parse(reader: &mut Events<&[u8]>) -> anyhow::Result<Self> {
        let mut items = Vec::new();

        while let Some(e) = reader.next() {
            let event = e?;
            match event {
                XmlEvent::EndElement { name } if name.local_name == "Items" => break,
                XmlEvent::StartElement { name, .. } if name.local_name == "CalendarItem" => {
                    let item = CalendarItem::parse(reader)?;
                    items.push(item);
                }
                _ => continue,
            }
        }
        Ok(items)
    }
}

impl XmlParser for CalendarItem {
    fn parse(reader: &mut Events<&[u8]>) -> anyhow::Result<Self> {
        let mut builder = CalendarItemBuilder::default();
        while let Some(e) = reader.next() {
            let event = e?;
            match event {
                XmlEvent::EndElement { name } if name.local_name == "CalendarItem" => break,
                XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == "ItemId" => {
                    if let Some(id) = attributes
                        .into_iter()
                        .find(|attr| attr.name.local_name == "Id")
                    {
                        builder.id(id.value);
                    }
                }
                XmlEvent::StartElement { name, .. } if name.local_name == "Subject" => {
                    if let Some(subject) = read_text("Subject", reader)? {
                        builder.subject(subject);
                    }
                }
                XmlEvent::StartElement { name, .. } if name.local_name == "Start" => {
                    if let Some(start_time) = read_text("Start", reader)? {
                        let time = DateTime::from_str(&start_time)?;
                        builder.start(time);
                    }
                }
                XmlEvent::StartElement { name, .. } if name.local_name == "End" => {
                    if let Some(time) = read_text("End", reader)? {
                        let time = DateTime::from_str(&time)?;
                        builder.end(time);
                    }
                }
                XmlEvent::StartElement { name, .. } if name.local_name == "Location" => {
                    let location = read_text("Location", reader)?;
                    builder.location(location);
                }
                _ => continue,
            }
        }
        let item = builder.build()?;
        Ok(item)
    }
}

fn read_text(element: &str, reader: &mut Events<&[u8]>) -> anyhow::Result<Option<String>> {
    let mut text = None;

    while let Some(e) = reader.next() {
        let event = e?;
        match event {
            XmlEvent::Characters(chars) => text = Some(chars),
            XmlEvent::EndElement { name } if name.local_name == element => break,
            _ => continue,
        }
    }

    Ok(text)
}

#[derive(Debug, Clone, Builder)]
pub struct Mailbox {
    pub name: String,
}
