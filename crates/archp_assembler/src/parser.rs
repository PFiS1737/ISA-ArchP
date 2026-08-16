mod expression;
mod identifier;
mod line;
mod operand;
mod string;

use std::num::ParseIntError;

pub use line::parse_line;
use nom::{
    AsChar, Input, Parser,
    character::complete::{char, space0},
    error::{FromExternalError, ParseError},
    sequence::delimited,
};

use crate::expression::EvalError;

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
    Eval(EvalError),

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

impl<'a> From<EvalError> for nom::Err<Error<'a>> {
    fn from(val: EvalError) -> Self {
        nom::Err::Failure(Error::Eval(val))
    }
}

pub type Result<'a, O> = nom::IResult<&'a str, O, Error<'a>>;
