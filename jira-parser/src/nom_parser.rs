use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take, take_till, take_till1, take_until, take_while1};
use nom::character::complete::{char, multispace0, multispace1, not_line_ending, one_of};
use nom::combinator::{eof, map, opt, peek};
use nom::error::ParseError;
use nom::multi::{many0, many1, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use nom::Parser;

use crate::ast;
use crate::ast::{TableField, Tag};

pub fn parse(input: &str) -> IResult<&str, Vec<ast::Tag>> {
    map(many0(parse_tag), simplify)(input)
}

fn parse_tag(input: &str) -> IResult<&str, ast::Tag> {
    alt((
        quote,
        panel,
        heading,
        unordered_list,
        ordered_list,
        newline,
        table,
        parse_inline_tag,
    ))(input)
}

fn parse_inline_tag(input: &str) -> IResult<&str, ast::Tag> {
    alt((
        strong,
        emphasis,
        citation,
        deleted,
        inserted,
        superscript,
        subscript,
        monospaced,
        inline_quote,
        link,
        image,
        color,
        icons,
        plain_text,
    ))(input)
}

fn icons(input: &str) -> IResult<&str, ast::Tag> {
    alt((
        icon_builder("/", ast::Icon::CheckMark),
        icon_builder("-", ast::Icon::Minus),
        icon_builder("!", ast::Icon::Warning),
    ))(input)
}

fn strong(input: &str) -> IResult<&str, ast::Tag> {
    inline_style('*', ast::Tag::Strong)(input)
}

fn emphasis(input: &str) -> IResult<&str, ast::Tag> {
    inline_style('_', ast::Tag::Emphasis)(input)
}

fn citation(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(tag("??"), is_not("??"), tag("??")),
        |text: &str| ast::Tag::Citation(text.into()),
    )(input)
}

fn deleted(input: &str) -> IResult<&str, ast::Tag> {
    inline_style('-', ast::Tag::Deleted)(input)
}

fn inserted(input: &str) -> IResult<&str, ast::Tag> {
    inline_style('+', ast::Tag::Inserted)(input)
}

fn superscript(input: &str) -> IResult<&str, ast::Tag> {
    inline_style('^', ast::Tag::Superscript)(input)
}

fn subscript(input: &str) -> IResult<&str, ast::Tag> {
    inline_style('~', ast::Tag::Subscript)(input)
}

fn monospaced(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(tag("{{"), is_not("}}"), tag("}}")),
        |text: &str| ast::Tag::Monospaced(text.into()),
    )(input)
}

fn inline_style(
    delimiter: char,
    tag: impl Fn(String) -> ast::Tag,
) -> impl FnMut(&str) -> IResult<&str, ast::Tag> {
    let abort_tokens = format!("{delimiter}\n");
    move |input| {
        map(
            delimited(
                char(delimiter),
                is_not(abort_tokens.as_str()),
                pair(
                    char(delimiter),
                    peek(alt((multispace1, newline_or_end_of_file))),
                ),
            ),
            |text: &str| tag(text.into()),
        )(input)
    }
}

fn inline_quote(input: &str) -> IResult<&str, ast::Tag> {
    map(
        preceded(tag("bq. "), take_till1(|c| c == '\n')),
        |text: &str| ast::Tag::InlineQuote(text.into()),
    )(input)
}

fn link(input: &str) -> IResult<&str, ast::Tag> {
    map(
        alt((
            delimited(
                tag("["),
                separated_pair(is_not("|"), char('|'), is_not("]")),
                tag("]"),
            ),
            map(delimited(tag("["), is_not("]"), tag("]")), |link| {
                (link, link)
            }),
        )),
        |(text, link): (&str, &str)| ast::Tag::Link(text.into(), link.into()),
    )(input)
}

fn heading(input: &str) -> IResult<&str, ast::Tag> {
    map(
        pair(
            delimited(char('h'), heading_level, tag(". ")),
            take_till1(|c| c == '\n').and_then(map(many1(parse_inline_tag), simplify)),
        ),
        |(level, content): (u8, Vec<ast::Tag>)| ast::Tag::Heading(level, content),
    )(input)
}

fn color(input: &str) -> IResult<&str, ast::Tag> {
    map(
        tuple((
            delimited(tag("{color:"), is_not("}"), char('}')),
            map(many0(is_not("{")), |lines: Vec<&str>| lines.join("\n")),
            tag("{color}"),
        )),
        |(color, text, _): (&str, _, &str)| ast::Tag::Color(color.into(), text.trim().into()),
    )(input)
}

fn quote(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(tag("{quote}"), whitespace(is_not("{")), tag("{quote}")),
        |text: &str| ast::Tag::Quote(text.trim().into()),
    )(input)
}

fn panel(input: &str) -> IResult<&str, ast::Tag> {
    map(
        tuple((
            delimited(
                tag("{panel"),
                take_until("}").and_then(parse_panel_options),
                pair(tag("}"), newline),
            ),
            map(whitespace(take_until("{panel}")), |text| text.trim()).and_then(parse),
            pair(
                tag("{panel}"),
                preceded(
                    many0(one_of(" \t")),
                    alt((map(newline, |_| ()), map(eof, |_| ()))),
                ),
            ),
        )),
        |(options, content, _)| ast::Tag::Panel(ast::Panel { content, ..options }),
    )(input)
}

fn image(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(
            char('!'),
            take_until("!").and_then(parse_image_options),
            char('!'),
        ),
        ast::Tag::Image,
    )(input)
}

fn parse_image_options(input: &str) -> IResult<&str, ast::Image> {
    map(
        tuple((
            take_till(|c| c == '|' || c == '!'),
            opt(char('|')),
            separated_list0(char('|'), parse_option),
        )),
        |(file, _, options)| {
            let mut image = ast::Image {
                filename: file.into(),
                ..Default::default()
            };
            for (key, value) in options {
                assign_image_option(&mut image, key, value);
            }
            image
        },
    )(input)
}

fn assign_image_option(image: &mut ast::Image, key: &str, value: &str) {
    match key {
        "width" => image.width = Some(value.trim().into()),
        "height" => image.height = Some(value.trim().into()),
        _ => {}
    }
}

fn parse_panel_options(input: &str) -> IResult<&str, ast::Panel> {
    alt((
        map(
            preceded(char(':'), separated_list1(char('|'), parse_option)),
            |further_options| {
                let mut panel = ast::Panel::default();
                for (key, value) in further_options {
                    assign_option(&mut panel, key, value);
                }
                panel
            },
        ),
        map(eof, |_| ast::Panel::default()),
    ))(input)
}

fn assign_option(panel: &mut ast::Panel, key: &str, value: &str) {
    match key {
        "title" => panel.title = Some(value.trim().into()),
        "borderStyle" => panel.border_style = Some(value.into()),
        "borderColor" => panel.border_color = Some(value.into()),
        "borderWidth" => panel.border_width = Some(value.into()),
        "bgColor" => panel.background_color = Some(value.into()),
        "titleBGColor" => panel.title_background_color = Some(value.into()),
        _ => {}
    }
}

fn parse_option(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        take_until("="),
        char('='),
        alt((take_until("|"), not_line_ending)),
    )(input)
}

fn heading_level(input: &str) -> IResult<&str, u8> {
    map(one_of("123456"), |c: char| match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        _ => unreachable!(),
    })(input)
}

fn plain_text(input: &str) -> IResult<&str, ast::Tag> {
    map(take(1usize), |text: &str| ast::Tag::Text(text.into()))(input)
}

fn newline(input: &str) -> IResult<&str, ast::Tag> {
    map(alt((tag("\n"), tag("\r\n"))), |_| ast::Tag::Newline)(input)
}

fn whitespace<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn newline_or_end_of_file(input: &str) -> IResult<&str, &str> {
    alt((tag("\n"), tag("\r\n"), eof))(input)
}

fn unordered_list(input: &str) -> IResult<&str, ast::Tag> {
    map(list("*"), ast::Tag::UnorderedList)(input)
}

fn ordered_list(input: &str) -> IResult<&str, ast::Tag> {
    map(list("#"), ast::Tag::OrderedList)(input)
}

fn list(separator: &'static str) -> impl FnMut(&str) -> IResult<&str, Vec<ast::ListItem>> {
    move |input| {
        map(
            separated_list1(
                nom::character::complete::newline,
                separated_pair(
                    map(many1(tag(separator)), |level| level.len()),
                    char(' '),
                    whitespace(take_till1(|c| c == '\n'))
                        .and_then(map(many1(parse_inline_tag), simplify)),
                ),
            ),
            |lines: Vec<(usize, Vec<ast::Tag>)>| {
                lines
                    .into_iter()
                    .map(|(level, content)| ast::ListItem {
                        level: level as u8,
                        content,
                    })
                    .collect()
            },
        )(input)
    }
}

fn icon_builder(icon: &str, icon_tag: ast::Icon) -> impl FnMut(&str) -> IResult<&str, ast::Tag> {
    let detector = format!("({icon})");

    move |input| map(tag(detector.as_str()), |_| ast::Tag::Icon(icon_tag))(input)
}

fn simplify(tags: Vec<ast::Tag>) -> Vec<ast::Tag> {
    tags.into_iter().fold(Vec::new(), |mut tags, tag| {
        match (tag, tags.pop()) {
            (ast::Tag::Text(next), Some(ast::Tag::Text(last))) => {
                tags.push(ast::Tag::Text(format!("{last}{next}")));
            }
            (ast::Tag::UnorderedList(mut next_items), Some(ast::Tag::UnorderedList(mut items))) => {
                items.append(&mut next_items);
                tags.push(ast::Tag::UnorderedList(items));
            }
            (next, Some(last)) => {
                tags.push(last);
                tags.push(next)
            }
            (next, None) => tags.push(next),
        }

        tags
    })
}

fn table(input: &str) -> IResult<&str, ast::Tag> {
    map(separated_list1(newline, table_row), |rows| {
        let rows = rows.into_iter().map(ast::TableRow).collect::<Vec<_>>();

        ast::Tag::Table(ast::Table { rows })
    })(input)
}

fn table_row(input: &str) -> IResult<&str, Vec<ast::TableField>> {
    preceded(
        peek(tag("|")),
        take_while1(|c| c != '\n').and_then(map(many1(parse_inline_tag), |tags| {
            let mut columns = vec![];
            let len = tags.len();
            let len = if tags.iter().nth(len - 2) == Some(&Tag::Text("|".into())) {
                len - 2
            } else {
                len - 1
            };
            let mut iterator = tags.into_iter().take(len).peekable();
            while let Some(tag) = iterator.next() {
                if is_column_separator(&tag) {
                    let mut tags = vec![];
                    let mut is_header = false;
                    if let Some(next) = iterator.peek() {
                        if is_column_separator(next) {
                            is_header = true;
                            iterator.next();
                        }
                    }
                    while let Some(tag) = iterator.peek() {
                        if is_column_separator(tag) {
                            break;
                        }
                        let tag = iterator.next().unwrap();
                        tags.push(tag);
                    }
                    let tags = simplify(tags);
                    let tags = tags
                        .into_iter()
                        .flat_map(|tag| match tag {
                            Tag::Text(text) => {
                                let x = text.clone();
                                let tags = if let Ok(("", tags)) =
                                    map(many1(parse_inline_tag), simplify)(&x)
                                {
                                    tags
                                } else {
                                    vec![Tag::Text(text)]
                                };
                                tags
                            }
                            tag => vec![tag],
                        })
                        .collect();
                    let field = if is_header {
                        TableField::Heading(tags)
                    } else {
                        TableField::Plain(tags)
                    };
                    columns.push(field);
                } else {
                    println!("no column_separator")
                }
            }

            columns
        })),
    )(input)
}

fn is_column_separator(tag: &ast::Tag) -> bool {
    &Tag::Text("|".to_string()) == tag
}

// fn table_row(input: &str) -> IResult<&str, Vec<ast::TableField>> {
//     preceded(
//         tag("|"),
//         take_while1(|c| c != '\n').and_then(many1(map(
//             pair(
//                 opt(tag("|")),
//                 terminated(take_until("|"), tag("|"))
//                     .and_then(map(many1(parse_inline_tag), simplify)),
//             ),
//             |(is_header, tags)| {
//                 if is_header.is_some() {
//                     ast::TableField::Heading(tags)
//                 } else {
//                     ast::TableField::Plain(tags)
//                 }
//             },
//         ))),
//     )(input)
// }
