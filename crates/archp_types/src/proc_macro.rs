use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Error, Ident, Result, Token,
    parse::{Parse, ParseStream},
};

use crate::{InstructionType, OperandType};

impl Parse for InstructionType {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            "R" => Ok(Self::R),
            "I" => Ok(Self::I),
            "B" => Ok(Self::B),
            "S" => Ok(Self::S),
            "U" => Ok(Self::U),
            "J" => Ok(Self::J),
            _ => Err(Error::new(ident.span(), "unknown instruction type")),
        }
    }
}

impl ToTokens for InstructionType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::R => quote!(R),
            Self::I => quote!(I),
            Self::B => quote!(B),
            Self::S => quote!(S),
            Self::U => quote!(U),
            Self::J => quote!(J),
        })
    }
}

impl Parse for OperandType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.parse::<Token![_]>().is_ok() {
            return Ok(Self::None);
        }

        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            "rs" => Ok(Self::RegS),
            "rd" => Ok(Self::RegD),
            s if let Some(n) = s.strip_prefix("imm") => {
                let n = n
                    .parse::<u8>()
                    .map_err(|_| Error::new(ident.span(), "invalid immediate size"))?;
                Ok(Self::Imm(n, true))
            },
            s if let Some(n) = s.strip_prefix("uimm") => {
                let n = n
                    .parse::<u8>()
                    .map_err(|_| Error::new(ident.span(), "invalid unsigned immediate size"))?;
                Ok(Self::Imm(n, false))
            },
            s if let Some(n) = s.strip_prefix("addr") => {
                let n = n
                    .parse::<u8>()
                    .map_err(|_| Error::new(ident.span(), "invalid address size"))?;
                Ok(Self::Addr(n))
            },
            _ => Err(Error::new(ident.span(), "unknown operand format type")),
        }
    }
}

impl ToTokens for OperandType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::RegS => quote!(RegS),
            Self::RegD => quote!(RegD),
            Self::Imm(n, signed) => quote!(Imm(#n, #signed)),
            Self::Addr(n) => quote!(Addr(#n)),
            Self::None => quote!(None),
        })
    }
}
