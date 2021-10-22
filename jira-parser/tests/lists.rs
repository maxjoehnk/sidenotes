use jira_parser::ast::*;
use jira_parser::parse;

#[test]
fn unordered_list_item() {
    let tag = parse("* Text").unwrap();

    assert_eq!(vec![Tag::UnorderedList(vec![ListItem::text("Text")])], tag)
}

#[test]
fn ordered_list_item() {
    let tag = parse("# Text").unwrap();

    assert_eq!(vec![Tag::OrderedList(vec![ListItem::text("Text")])], tag)
}

#[test]
fn nested_list_item() {
    let input = r#"* Top Level
** 1st level
*** 2nd level
**** 3rd level"#;

    let tag = parse(input).unwrap();

    assert_eq!(
        vec![Tag::UnorderedList(vec![
            ListItem {
                level: 1,
                content: vec![Tag::Text("Top Level".into())]
            },
            ListItem {
                level: 2,
                content: vec![Tag::Text("1st level".into())]
            },
            ListItem {
                level: 3,
                content: vec![Tag::Text("2nd level".into())]
            },
            ListItem {
                level: 4,
                content: vec![Tag::Text("3rd level".into())]
            },
        ])],
        tag
    )
}
