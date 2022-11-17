use jira_parser::ast::*;
use jira_parser::*;

#[test]
fn simple_panel() {
    let input = r#"{panel}
    Some content
{panel}"#;
    let expected = "Some content";

    let tag = parse(input).unwrap();

    assert_eq!(
        vec![Tag::Panel(Panel {
            content: vec![Tag::Text(expected.into())],
            ..Default::default()
        })],
        tag
    );
}

#[test]
fn panel_with_title() {
    let input = r#"{panel:title=Test Title}
    Some content
    {panel}"#;
    let expected = "Some content";

    let tag = parse(input).unwrap();

    assert_eq!(
        vec![Tag::Panel(Panel {
            content: vec![Tag::Text(expected.into())],
            title: Some("Test Title".into()),
            ..Default::default()
        })],
        tag
    );
}

#[test]
fn panel_with_trailing_spaces() {
    let input = r#"{panel:title=Test Title}
Some content
{panel}   "#;

    let tag = parse(input).unwrap();

    assert_eq!(
        vec![Tag::Panel(Panel {
            content: vec![Tag::Text("Some content".into())],
            title: Some("Test Title".into()),
            ..Default::default()
        })],
        tag
    );
}

#[test]
fn panel_with_trailing_spaces_before_newline() {
    let input = r#"{panel:title=Test Title}
Some content
{panel}   
"#;

    let tag = parse(input).unwrap();

    assert_eq!(
        vec![Tag::Panel(Panel {
            content: vec![Tag::Text("Some content".into())],
            title: Some("Test Title".into()),
            ..Default::default()
        })],
        tag
    );
}
