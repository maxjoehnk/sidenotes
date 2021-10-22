use test_case::test_case;

use jira_parser::ast::*;
use jira_parser::*;

#[test_case("h1. Biggest heading", 1)]
#[test_case("h2. Bigger heading", 2)]
#[test_case("h3. Big heading", 3)]
#[test_case("h4. Normal heading", 4)]
#[test_case("h5. Small heading", 5)]
#[test_case("h6. Smallest heading", 6)]
fn heading(input: &'static str, level: u8) {
    let tag = parse(input).unwrap();

    assert_eq!(
        vec![Tag::Heading(
            level,
            Tag::Text(input[4..].to_string()).into()
        )],
        tag
    );
}

#[test]
fn plain_text() {
    let input = "test";
    let tag = parse(input).unwrap();

    assert_eq!(vec![Tag::Text(input.to_string())], tag)
}

#[test]
fn text_effect_strong() {
    let tag = parse("*strong*").unwrap();

    assert_eq!(vec![Tag::Strong("strong".into())], tag);
}

#[test]
fn text_effect_emphasis() {
    let tag = parse("_emphasis_").unwrap();

    assert_eq!(vec![Tag::Emphasis("emphasis".into())], tag);
}

#[test]
fn text_effect_citation() {
    let tag = parse("??citation??").unwrap();

    assert_eq!(vec![Tag::Citation("citation".into())], tag);
}

#[test]
fn text_effect_deleted() {
    let tag = parse("-deleted-").unwrap();

    assert_eq!(vec![Tag::Deleted("deleted".into())], tag);
}

#[test]
fn text_effect_inserted() {
    let tag = parse("+inserted+").unwrap();

    assert_eq!(vec![Tag::Inserted("inserted".into())], tag);
}

#[test]
fn text_effect_superscript() {
    let tag = parse("^superscript^").unwrap();

    assert_eq!(vec![Tag::Superscript("superscript".into())], tag);
}

#[test]
fn text_effect_subscript() {
    let tag = parse("~subscript~").unwrap();

    assert_eq!(vec![Tag::Subscript("subscript".into())], tag);
}

#[test]
fn text_effect_monospaced() {
    let tag = parse("{{monospaced}}").unwrap();

    assert_eq!(vec![Tag::Monospaced("monospaced".into())], tag);
}

#[test]
fn text_effect_inline_quote() {
    let tag = parse("bq. Some block quoted text").unwrap();

    assert_eq!(vec![Tag::InlineQuote("Some block quoted text".into())], tag);
}

#[test]
fn quote() {
    let input = r#"{quote}
        here is quotable
    content to be quoted
    {quote}"#;
    let expected = r#"here is quotable
    content to be quoted"#;

    let tag = parse(input).unwrap();

    assert_eq!(vec![Tag::Quote(expected.into())], tag);
}

#[test]
fn color() {
    let input = r#"{color:red}
        look ma, red text!
    {color}"#;

    let tag = parse(input).unwrap();

    assert_eq!(
        vec![Tag::Color("red".into(), "look ma, red text!".into())],
        tag
    );
}
