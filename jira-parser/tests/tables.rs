use jira_parser::{ast, parse};

#[test]
fn table() {
    let input = r#"||heading 1||heading 2||heading 3||
|cell A1|cell A2|cell A3|
|cell B1|cell B2|cell B3|"#;
    let expected = vec![ast::Tag::Table(ast::Table {
        rows: vec![
            vec![
                ast::TableField::Heading(vec![ast::Tag::Text("heading 1".into())]),
                ast::TableField::Heading(vec![ast::Tag::Text("heading 2".into())]),
                ast::TableField::Heading(vec![ast::Tag::Text("heading 3".into())]),
            ]
            .into(),
            vec![
                ast::TableField::Plain(vec![ast::Tag::Text("cell A1".into())]),
                ast::TableField::Plain(vec![ast::Tag::Text("cell A2".into())]),
                ast::TableField::Plain(vec![ast::Tag::Text("cell A3".into())]),
            ]
            .into(),
            vec![
                ast::TableField::Plain(vec![ast::Tag::Text("cell B1".into())]),
                ast::TableField::Plain(vec![ast::Tag::Text("cell B2".into())]),
                ast::TableField::Plain(vec![ast::Tag::Text("cell B3".into())]),
            ]
            .into(),
        ],
    })];

    let tags = parse(input).unwrap();

    assert_eq!(tags, expected)
}

#[test]
fn heading_column() {
    let input = r#"||label 1|cell B1|
||label 2|cell B2|
||label 3|cell B3|"#;
    let expected = vec![ast::Tag::Table(ast::Table {
        rows: vec![
            vec![
                ast::TableField::Heading(vec![ast::Tag::Text("label 1".into())]),
                ast::TableField::Plain(vec![ast::Tag::Text("cell B1".into())]),
            ]
            .into(),
            vec![
                ast::TableField::Heading(vec![ast::Tag::Text("label 2".into())]),
                ast::TableField::Plain(vec![ast::Tag::Text("cell B2".into())]),
            ]
            .into(),
            vec![
                ast::TableField::Heading(vec![ast::Tag::Text("label 3".into())]),
                ast::TableField::Plain(vec![ast::Tag::Text("cell B3".into())]),
            ]
            .into(),
        ],
    })];

    let tags = parse(input).unwrap();

    assert_eq!(tags, expected)
}

#[test]
fn markup_in_fields() {
    let input = r#"||link|[Atlassian|https://atlassian.com]|
||strong|*bold*|
||Image|!image.jpg|width=200!|"#;
    let expected = vec![ast::Tag::Table(ast::Table {
        rows: vec![
            vec![
                ast::TableField::Heading(vec![ast::Tag::Text("link".into())]),
                ast::TableField::Plain(vec![ast::Tag::Link(
                    "Atlassian".into(),
                    "https://atlassian.com".into(),
                )]),
            ]
            .into(),
            vec![
                ast::TableField::Heading(vec![ast::Tag::Text("strong".into())]),
                ast::TableField::Plain(vec![ast::Tag::Strong("bold".into())]),
            ]
            .into(),
            vec![
                ast::TableField::Heading(vec![ast::Tag::Text("Image".into())]),
                ast::TableField::Plain(vec![ast::Tag::Image(ast::Image {
                    filename: "image.jpg".into(),
                    width: Some("200".into()),
                    ..Default::default()
                })]),
            ]
            .into(),
        ],
    })];

    let tags = parse(input).unwrap();

    assert_eq!(tags, expected)
}
