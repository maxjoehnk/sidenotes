pub mod ast;
mod nom_parser;

pub fn parse(input: &str) -> anyhow::Result<Vec<ast::Tag>> {
    let (remainder, tags) = nom_parser::parse(input).unwrap();

    assert!(remainder.is_empty());

    Ok(tags)
}

