pub mod address;
pub mod expression;
pub mod immediate;
pub mod register;
pub mod types;

use nom::{
    AsChar, Input, Parser, character::complete::multispace0, error::ParseError, sequence::delimited,
};

fn ws<I, O, E: ParseError<I>, F>(inner: F) -> impl Parser<I, Output = O, Error = E>
where
    F: Parser<I, Output = O, Error = E>,
    I: Input<Item: AsChar>,
{
    delimited(multispace0, inner, multispace0)
}
