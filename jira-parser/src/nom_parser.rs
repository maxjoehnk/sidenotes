use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take, take_till, take_till1, take_until};
use nom::character::complete::{char, multispace0, not_line_ending, one_of};
use nom::combinator::{eof, map, opt};
use nom::error::ParseError;
use nom::multi::{many0, many1, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use nom::Parser;

use crate::ast;

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
        icon_builder("/", ast::Icon::CheckMark),
        icon_builder("-", ast::Icon::Minus),
        icon_builder("!", ast::Icon::Warning),
        plain_text,
    ))(input)
}

fn strong(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(char('*'), is_not("*"), char('*')),
        |text: &str| ast::Tag::Strong(text.into()),
    )(input)
}

fn emphasis(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(char('_'), is_not("_"), char('_')),
        |text: &str| ast::Tag::Emphasis(text.into()),
    )(input)
}

fn citation(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(tag("??"), is_not("??"), tag("??")),
        |text: &str| ast::Tag::Citation(text.into()),
    )(input)
}

fn deleted(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(char('-'), is_not("-"), char('-')),
        |text: &str| ast::Tag::Deleted(text.into()),
    )(input)
}

fn inserted(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(char('+'), is_not("+"), char('+')),
        |text: &str| ast::Tag::Inserted(text.into()),
    )(input)
}

fn superscript(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(char('^'), is_not("^"), char('^')),
        |text: &str| ast::Tag::Superscript(text.into()),
    )(input)
}

fn subscript(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(char('~'), is_not("~"), char('~')),
        |text: &str| ast::Tag::Subscript(text.into()),
    )(input)
}

fn monospaced(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(tag("{{"), is_not("}}"), tag("}}")),
        |text: &str| ast::Tag::Monospaced(text.into()),
    )(input)
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
