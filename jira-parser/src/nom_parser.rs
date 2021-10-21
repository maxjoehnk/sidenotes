use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_till1, take_until};
use nom::character::complete::{char, one_of, multispace0, not_line_ending};
use nom::combinator::{map, eof};
use nom::IResult;
use nom::sequence::{delimited, pair, preceded, tuple, separated_pair};
use nom::multi::{many0, many1};
use nom::Parser;
use nom::error::ParseError;

use crate::ast;

pub fn parse(input: &str) -> IResult<&str, Vec<ast::Tag>> {
    many0(parse_tag)(input)
}

fn parse_tag(input: &str) -> IResult<&str, ast::Tag> {
    alt((
        color,
        quote,
        panel,
        heading,
        newline,
        parse_inline_tag
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
        plain_text,
    ))(input)
}

fn strong(input: &str) -> IResult<&str, ast::Tag> {
    map(delimited(char('*'), is_not("*"), char('*')), |text: &str| ast::Tag::Strong(text.into()))(input)
}

fn emphasis(input: &str) -> IResult<&str, ast::Tag> {
    map(delimited(char('_'), is_not("_"), char('_')), |text: &str| ast::Tag::Emphasis(text.into()))(input)
}

fn citation(input: &str) -> IResult<&str, ast::Tag> {
    map(delimited(tag("??"), is_not("??"), tag("??")), |text: &str| ast::Tag::Citation(text.into()))(input)
}

fn deleted(input: &str) -> IResult<&str, ast::Tag> {
    map(delimited(char('-'), is_not("-"), char('-')), |text: &str| ast::Tag::Deleted(text.into()))(input)
}

fn inserted(input: &str) -> IResult<&str, ast::Tag> {
    map(delimited(char('+'), is_not("+"), char('+')), |text: &str| ast::Tag::Inserted(text.into()))(input)
}

fn superscript(input: &str) -> IResult<&str, ast::Tag> {
    map(delimited(char('^'), is_not("^"), char('^')), |text: &str| ast::Tag::Superscript(text.into()))(input)
}

fn subscript(input: &str) -> IResult<&str, ast::Tag> {
    map(delimited(char('~'), is_not("~"), char('~')), |text: &str| ast::Tag::Subscript(text.into()))(input)
}

fn monospaced(input: &str) -> IResult<&str, ast::Tag> {
    map(delimited(tag("{{"), is_not("}}"), tag("}}")), |text: &str| ast::Tag::Monospaced(text.into()))(input)
}

fn inline_quote(input: &str) -> IResult<&str, ast::Tag> {
    map(preceded(tag("bq. "), take_till1(|c| c == '\n')), |text: &str| ast::Tag::InlineQuote(text.into()))(input)
}

fn heading(input: &str) -> IResult<&str, ast::Tag> {
    map(
        pair(delimited(char('h'), heading_level, tag(". ")), take_till1(|c| c == '\n').and_then(many1(parse_inline_tag))),
        |(level, content): (u8, Vec<ast::Tag>)| ast::Tag::Heading(level, content)
    )(input)
}

fn color(input: &str) -> IResult<&str, ast::Tag> {
    map(
        tuple((
            delimited(tag("{color:"), is_not("}"), char('}')),
            map(many0(is_not("{")), |lines: Vec<&str>| lines.join("\n")),
            tag("{color}")
        )),
        |(color, text, _): (&str, _, &str)| ast::Tag::Color(color.into(), text.trim().into()),
    )(input)
}

fn quote(input: &str) -> IResult<&str, ast::Tag> {
    map(
        delimited(tag("{quote}"), ws(is_not("{")), tag("{quote}")),
        |text: &str| ast::Tag::Quote(text.trim().into())
    )(input)
}

fn panel(input: &str) -> IResult<&str, ast::Tag> {
    map(
        tuple((
            delimited(tag("{panel"), take_until("}").and_then(parse_panel_options), pair(tag("}"), newline)),
            map(ws(is_not("{")), |text| text.trim()).and_then(parse),
            pair(tag("{panel}"), alt((map(newline, |_| ()), map(eof, |_| ()))))
        )),
        |(options, content, _)| ast::Tag::Panel(ast::Panel {
            content,
            ..options
        })
    )(input)
}

fn parse_panel_options(input: &str) -> IResult<&str, ast::Panel> {
    alt((
        map(preceded(char(':'), pair(
            parse_option,
            many0(preceded(char('|'), parse_option))
        )), |(option, further_options)| {
            let mut panel = ast::Panel::default();
            assign_option(&mut panel, option.0, option.1);
            for (key, value) in further_options {
                assign_option(&mut panel, key, value);
            }
            panel
        }),
        map(eof, |_| ast::Panel::default())
    ))(input)
}

fn assign_option(panel: &mut ast::Panel, key: &str, value: &str) {
    match key {
        "title" => panel.title = Some(value.into()),
        "borderStyle" => panel.border_style = Some(value.into()),
        "borderColor" => panel.border_color = Some(value.into()),
        "borderWidth" => panel.border_width = Some(value.into()),
        "bgColor" => panel.background_color = Some(value.into()),
        "titleBgColor" => panel.title_background_color = Some(value.into()),
        _ => {},
    }
}

fn parse_option(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(take_until("="), char('='), alt((
        take_until("|"),
        not_line_ending
    )))(input)
}

fn heading_level(input: &str) -> IResult<&str, u8> {
    map(
        one_of("123456"),
        |c: char|
            match c {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                _ => unreachable!()
            },
    )(input)
}

fn plain_text(input: &str) -> IResult<&str, ast::Tag> {
    map(take_till1(|c| c == '\n'), |text: &str| ast::Tag::Text(text.into()))(input)
}

fn newline(input: &str) -> IResult<&str, ast::Tag> {
    map(char('\n'), |_| ast::Tag::Newline)(input)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(
        multispace0,
        inner,
        multispace0
    )
}
