use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"]
pub struct SyntaxParser;

pub fn parse(text: &str) -> Result<Pair<Rule>, Error<Rule>> {
    let mut pairs = SyntaxParser::parse(Rule::program, &text)?;
    Ok(pairs.next().unwrap())
}
