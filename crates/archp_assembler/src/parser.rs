pub mod address;
pub mod expression;
pub mod grammar;
pub mod immediate;
pub mod operand;
pub mod register;
pub mod types;

use nom::{
    AsChar, IResult, Input, Parser,
    bytes::complete::take_while,
    character::{
        complete::{char, space0},
        satisfy,
    },
    combinator::recognize,
    error::ParseError,
    sequence::delimited,
};

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

fn ident(input: &str) -> IResult<&str, &str> {
    recognize((
        satisfy(|c| c.is_ascii_alphabetic() || c == '_' || c == '.'),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '.'),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::{
        Err,
        error::{Error, ErrorKind},
    };

    use super::*;

    #[test]
    fn test_ident() {
        assert_eq!(ident("a1"), Ok(("", "a1")));
        assert_eq!(ident("abc123"), Ok(("", "abc123")));
        assert_eq!(ident("_abc"), Ok(("", "_abc")));
        assert_eq!(ident(".abc"), Ok(("", ".abc")));
        assert_eq!(ident("abc_123"), Ok(("", "abc_123")));
        assert_eq!(ident("abc.123"), Ok(("", "abc.123")));
        assert_eq!(
            ident("123abc"),
            Err(Err::Error(Error::new("123abc", ErrorKind::Satisfy)))
        );
    }
}
