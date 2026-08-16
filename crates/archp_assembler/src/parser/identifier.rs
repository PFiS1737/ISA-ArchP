use nom::{
    Parser, bytes::complete::take_while, character::complete::satisfy, combinator::recognize,
};

use crate::parser::Result;

pub fn ident(input: &str) -> Result<'_, &str> {
    recognize((
        satisfy(|c| c.is_ascii_alphabetic() || c == '_' || c == '.' || c == '$'),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '$'),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Error;

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
