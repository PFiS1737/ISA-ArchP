use syn::{
    Attribute, Ident, LitInt, Result, Token, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::instructions::{Instruction, Opcode, Root};

impl Parse for Root {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            opcodes: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Parse for Opcode {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = Attribute::parse_outer(input)?;

        let opcode = input.parse()?;

        input.parse::<Token![:]>()?;

        let content;
        braced!(content in input);

        let instructions = Punctuated::parse_terminated(&content)?;

        Ok(Self {
            attrs,
            opcode,
            instructions,
        })
    }
}

impl Parse for Instruction {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = Attribute::parse_outer(input)?;

        let funct3 = if input.peek(LitInt) {
            let funct3 = input.parse()?;

            input.parse::<Token![:]>()?;

            Some(funct3)
        } else {
            None
        };

        let itype = input.parse()?;

        input.parse::<Token![=>]>()?;

        let name = input.parse()?;

        let formats = if input.peek(Ident) || input.peek(Token![_]) {
            Some(Punctuated::parse_separated_nonempty(input)?)
        } else {
            None
        };

        Ok(Instruction {
            attrs,
            funct3,
            itype,
            name,
            formats,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use proc_macro2::TokenStream;
    use quote::quote;

    use super::*;

    fn parse(tokens: TokenStream) -> Root {
        syn::parse2(tokens).unwrap()
    }

    #[test]
    fn parses_single_opcode_with_funct3() {
        let tokens = quote! {
            0b0000000 : {
                0: R => add rd, _, rs;
                0: R => sub _, rs, rs;
                2: I => addi
            };
        };

        assert_debug_snapshot!(parse(tokens), @"
        Root {
            opcodes: [
                Opcode {
                    attrs: [],
                    opcode: LitInt {
                        token: 0b0000000,
                    },
                    instructions: [
                        Instruction {
                            attrs: [],
                            funct3: Some(
                                LitInt {
                                    token: 0,
                                },
                            ),
                            itype: R,
                            name: Ident(
                                add,
                            ),
                            formats: Some(
                                [
                                    RegD,
                                    Token![,],
                                    None,
                                    Token![,],
                                    RegS,
                                ],
                            ),
                        },
                        Token![;],
                        Instruction {
                            attrs: [],
                            funct3: Some(
                                LitInt {
                                    token: 0,
                                },
                            ),
                            itype: R,
                            name: Ident(
                                sub,
                            ),
                            formats: Some(
                                [
                                    None,
                                    Token![,],
                                    RegS,
                                    Token![,],
                                    RegS,
                                ],
                            ),
                        },
                        Token![;],
                        Instruction {
                            attrs: [],
                            funct3: Some(
                                LitInt {
                                    token: 2,
                                },
                            ),
                            itype: I,
                            name: Ident(
                                addi,
                            ),
                            formats: None,
                        },
                    ],
                },
                Token![;],
            ],
        }
        ");
    }

    #[test]
    fn parses_single_opcode_without_funct3() {
        let tokens = quote! {
            0b0000001: {
                U => lui rd, uimm20;
            };
        };

        assert_debug_snapshot!(parse(tokens), @"
        Root {
            opcodes: [
                Opcode {
                    attrs: [],
                    opcode: LitInt {
                        token: 0b0000001,
                    },
                    instructions: [
                        Instruction {
                            attrs: [],
                            funct3: None,
                            itype: U,
                            name: Ident(
                                lui,
                            ),
                            formats: Some(
                                [
                                    RegD,
                                    Token![,],
                                    Imm(
                                        20,
                                        false,
                                    ),
                                ],
                            ),
                        },
                        Token![;],
                    ],
                },
                Token![;],
            ],
        }
        ");
    }

    #[test]
    fn parses_multiple_opcodes_mixed() {
        let tokens = quote! {
            0b0000000: {
                0: R => add rd, rs, rs;
            };
            0b0000001: {
                U => lui rd, uimm20;
            };
            0b0000010: {
                0: I => lw rd, rs, imm12;
            };
        };

        assert_debug_snapshot!(parse(tokens), @"
        Root {
            opcodes: [
                Opcode {
                    attrs: [],
                    opcode: LitInt {
                        token: 0b0000000,
                    },
                    instructions: [
                        Instruction {
                            attrs: [],
                            funct3: Some(
                                LitInt {
                                    token: 0,
                                },
                            ),
                            itype: R,
                            name: Ident(
                                add,
                            ),
                            formats: Some(
                                [
                                    RegD,
                                    Token![,],
                                    RegS,
                                    Token![,],
                                    RegS,
                                ],
                            ),
                        },
                        Token![;],
                    ],
                },
                Token![;],
                Opcode {
                    attrs: [],
                    opcode: LitInt {
                        token: 0b0000001,
                    },
                    instructions: [
                        Instruction {
                            attrs: [],
                            funct3: None,
                            itype: U,
                            name: Ident(
                                lui,
                            ),
                            formats: Some(
                                [
                                    RegD,
                                    Token![,],
                                    Imm(
                                        20,
                                        false,
                                    ),
                                ],
                            ),
                        },
                        Token![;],
                    ],
                },
                Token![;],
                Opcode {
                    attrs: [],
                    opcode: LitInt {
                        token: 0b0000010,
                    },
                    instructions: [
                        Instruction {
                            attrs: [],
                            funct3: Some(
                                LitInt {
                                    token: 0,
                                },
                            ),
                            itype: I,
                            name: Ident(
                                lw,
                            ),
                            formats: Some(
                                [
                                    RegD,
                                    Token![,],
                                    RegS,
                                    Token![,],
                                    Imm(
                                        12,
                                        true,
                                    ),
                                ],
                            ),
                        },
                        Token![;],
                    ],
                },
                Token![;],
            ],
        }
        ");
    }

    #[test]
    fn parses_attributes() {
        let tokens = quote! {
            /// doc comments
            0b0000000: {
                /// doc comments
                #[cfg(feature = "foo")]
                0: R => add rd, rs, rs;

                /// doc comments
                2: I => addi;
            };

            /// doc comments
            #[cfg(feature = "bar")]
            0b0000001: {
                /// doc comments
                U => lui rd, uimm20;
            };
        };

        assert_debug_snapshot!(parse(tokens), @r#"
        Root {
            opcodes: [
                Opcode {
                    attrs: [
                        Attribute {
                            pound_token: Token![#],
                            style: AttrStyle::Outer,
                            bracket_token: Bracket,
                            meta: Meta::NameValue {
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident(
                                                doc,
                                            ),
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                                eq_token: Token![=],
                                value: Expr::Lit {
                                    attrs: [],
                                    lit: Lit::Str {
                                        token: r" doc comments",
                                    },
                                },
                            },
                        },
                    ],
                    opcode: LitInt {
                        token: 0b0000000,
                    },
                    instructions: [
                        Instruction {
                            attrs: [
                                Attribute {
                                    pound_token: Token![#],
                                    style: AttrStyle::Outer,
                                    bracket_token: Bracket,
                                    meta: Meta::NameValue {
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident(
                                                        doc,
                                                    ),
                                                    arguments: PathArguments::None,
                                                },
                                            ],
                                        },
                                        eq_token: Token![=],
                                        value: Expr::Lit {
                                            attrs: [],
                                            lit: Lit::Str {
                                                token: r" doc comments",
                                            },
                                        },
                                    },
                                },
                                Attribute {
                                    pound_token: Token![#],
                                    style: AttrStyle::Outer,
                                    bracket_token: Bracket,
                                    meta: Meta::List {
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident(
                                                        cfg,
                                                    ),
                                                    arguments: PathArguments::None,
                                                },
                                            ],
                                        },
                                        delimiter: MacroDelimiter::Paren(
                                            Paren,
                                        ),
                                        tokens: TokenStream [
                                            Ident {
                                                sym: feature,
                                            },
                                            Punct {
                                                char: '=',
                                                spacing: Alone,
                                            },
                                            Literal {
                                                lit: "foo",
                                            },
                                        ],
                                    },
                                },
                            ],
                            funct3: Some(
                                LitInt {
                                    token: 0,
                                },
                            ),
                            itype: R,
                            name: Ident(
                                add,
                            ),
                            formats: Some(
                                [
                                    RegD,
                                    Token![,],
                                    RegS,
                                    Token![,],
                                    RegS,
                                ],
                            ),
                        },
                        Token![;],
                        Instruction {
                            attrs: [
                                Attribute {
                                    pound_token: Token![#],
                                    style: AttrStyle::Outer,
                                    bracket_token: Bracket,
                                    meta: Meta::NameValue {
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident(
                                                        doc,
                                                    ),
                                                    arguments: PathArguments::None,
                                                },
                                            ],
                                        },
                                        eq_token: Token![=],
                                        value: Expr::Lit {
                                            attrs: [],
                                            lit: Lit::Str {
                                                token: r" doc comments",
                                            },
                                        },
                                    },
                                },
                            ],
                            funct3: Some(
                                LitInt {
                                    token: 2,
                                },
                            ),
                            itype: I,
                            name: Ident(
                                addi,
                            ),
                            formats: None,
                        },
                        Token![;],
                    ],
                },
                Token![;],
                Opcode {
                    attrs: [
                        Attribute {
                            pound_token: Token![#],
                            style: AttrStyle::Outer,
                            bracket_token: Bracket,
                            meta: Meta::NameValue {
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident(
                                                doc,
                                            ),
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                                eq_token: Token![=],
                                value: Expr::Lit {
                                    attrs: [],
                                    lit: Lit::Str {
                                        token: r" doc comments",
                                    },
                                },
                            },
                        },
                        Attribute {
                            pound_token: Token![#],
                            style: AttrStyle::Outer,
                            bracket_token: Bracket,
                            meta: Meta::List {
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident(
                                                cfg,
                                            ),
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                                delimiter: MacroDelimiter::Paren(
                                    Paren,
                                ),
                                tokens: TokenStream [
                                    Ident {
                                        sym: feature,
                                    },
                                    Punct {
                                        char: '=',
                                        spacing: Alone,
                                    },
                                    Literal {
                                        lit: "bar",
                                    },
                                ],
                            },
                        },
                    ],
                    opcode: LitInt {
                        token: 0b0000001,
                    },
                    instructions: [
                        Instruction {
                            attrs: [
                                Attribute {
                                    pound_token: Token![#],
                                    style: AttrStyle::Outer,
                                    bracket_token: Bracket,
                                    meta: Meta::NameValue {
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident(
                                                        doc,
                                                    ),
                                                    arguments: PathArguments::None,
                                                },
                                            ],
                                        },
                                        eq_token: Token![=],
                                        value: Expr::Lit {
                                            attrs: [],
                                            lit: Lit::Str {
                                                token: r" doc comments",
                                            },
                                        },
                                    },
                                },
                            ],
                            funct3: None,
                            itype: U,
                            name: Ident(
                                lui,
                            ),
                            formats: Some(
                                [
                                    RegD,
                                    Token![,],
                                    Imm(
                                        20,
                                        false,
                                    ),
                                ],
                            ),
                        },
                        Token![;],
                    ],
                },
                Token![;],
            ],
        }
        "#);
    }
}
