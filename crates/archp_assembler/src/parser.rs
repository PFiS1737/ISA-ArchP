pub mod expression;
pub mod line;
pub mod operand;
pub mod types;

use std::num::ParseIntError;

use nom::{
    AsChar, Input, Parser,
    bytes::complete::take_while,
    character::{
        complete::{char, space0},
        satisfy,
    },
    combinator::recognize,
    error::{FromExternalError, ParseError},
    sequence::delimited,
};

use crate::parser::types::expression::EvalError;

fn ws<I, O, E: ParseError<I>, F>(inner: F) -> impl Parser<I, Output = O, Error = E>
where
    F: Parser<I, Output = O, Error = E>,
    I: Input<Item: AsChar>,
{
    delimited(space0, inner, space0)
}

fn parens<I, O, E: ParseError<I>, F>(inner: F) -> impl Parser<I, Output = O, Error = E>
where
    F: Parser<I, Output = O, Error = E>,
    I: Input<Item: AsChar>,
{
    delimited(ws(char('(')), inner, ws(char(')')))
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Error<'a> {
    #[error("Evaluation error: {0}")]
    Eval(EvalError<'a>),

    #[error("Nom error: {0}")]
    Nom(nom::error::Error<&'a str>),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] ParseIntError),
}

impl<'a> ParseError<&'a str> for Error<'a> {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        Error::Nom(nom::error::Error::new(input, kind))
    }

    fn append(_: &'a str, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> FromExternalError<&'a str, ParseIntError> for Error<'a> {
    fn from_external_error(_: &'a str, _: nom::error::ErrorKind, e: ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl<'a> From<EvalError<'a>> for nom::Err<Error<'a>> {
    fn from(val: EvalError<'a>) -> Self {
        nom::Err::Failure(Error::Eval(val))
    }
}

pub type Result<'a, O> = nom::IResult<&'a str, O, Error<'a>>;

fn ident(input: &str) -> Result<'_, &str> {
    recognize((
        satisfy(|c| c.is_ascii_alphabetic() || c == '_' || c == '.' || c == '$'),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '$'),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ident() {
        assert_eq!(ident("a1"), Ok(("", "a1")));
        assert_eq!(ident("abc1$23"), Ok(("", "abc1$23")));
        assert_eq!(ident("_abc"), Ok(("", "_abc")));
        assert_eq!(ident(".abc"), Ok(("", ".abc")));
        assert_eq!(ident("ab$c_123"), Ok(("", "ab$c_123")));
        assert_eq!(ident("abc.123"), Ok(("", "abc.123")));
        assert_eq!(
            ident("123abc"),
            Err(nom::Err::Error(Error::Nom(nom::error::Error::new(
                "123abc",
                nom::error::ErrorKind::Satisfy
            ))))
        );
    }
}
