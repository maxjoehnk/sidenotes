use jira_parser::ast::*;
use jira_parser::parse;

#[test]
fn combine_format() {
    let tags = parse("*bold*_italic_-deleted-+inserted+").unwrap();

    assert_eq!(
        vec![
            Tag::Strong("bold".into()),
            Tag::Emphasis("italic".into()),
            Tag::Deleted("deleted".into()),
            Tag::Inserted("inserted".into()),
        ],
        tags
    )
}

#[test]
fn formatted_heading() {
    let tags = parse("h1. *Important* something something").unwrap();

    assert_eq!(
        vec![Tag::Heading(
            1,
            vec![
                Tag::Strong("Important".into()),
                Tag::Text(" something something".into()).into(),
            ]
        )],
        tags
    );
}

#[test]
fn user_story() {
    let text = r#"{panel:bgColor=#eae6ff}
h3. *User-Story*
*As a* User
*I want to be able* to view formatted jira tickets
*in order to* better comprehend the ticket content.
{panel}

{panel:bgColor=#e3fcef}
h3. *Acceptance criteria*
* text effects are supported
* panels are supported
* lists are supported
** nested lists as well
{panel}"#;
    let expected = vec![
        Tag::Panel(Panel {
            content: vec![
                Tag::Heading(3, Tag::Strong("User-Story".into()).into()),
                Tag::Newline,
                Tag::Strong("As a".into()),
                Tag::Text(" User".into()),
                Tag::Newline,
                Tag::Strong("I want to be able".into()),
                Tag::Text(" to view formatted jira tickets".into()),
                Tag::Newline,
                Tag::Strong("in order to".into()),
                Tag::Text(" better comprehend the ticket content.".into()),
            ],
            background_color: Some("#eae6ff".into()),
            ..Default::default()
        }),
        Tag::Newline,
        Tag::Panel(Panel {
            content: vec![
                Tag::Heading(3, Tag::Strong("Acceptance criteria".into()).into()),
                Tag::Newline,
                Tag::UnorderedList(vec![
                    ListItem::text("text effects are supported"),
                    ListItem::text("panels are supported"),
                    ListItem::text("lists are supported"),
                    ListItem {
                        level: 2,
                        content: vec![Tag::Text("nested lists as well".into())],
                    },
                ]),
            ],
            background_color: Some("#e3fcef".into()),
            ..Default::default()
        }),
    ];

    let tags = parse(text).unwrap();

    assert_eq!(expected, tags);
}
