use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Result, parse_quote, parse_str};

use crate::instructions::{Instruction, Opcode, Root};

impl Root {
    pub fn generate(self) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();

        tokens.extend(self.clone().generate_definitions()?);
        tokens.extend(self.clone().generate_name_matcher()?);
        tokens.extend(self.clone().generate_opcode_matcher()?);

        Ok(tokens)
    }

    fn generate_definitions(self) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();

        for Opcode {
            attrs: attrs_opcode,
            opcode,
            instructions,
        } in self.opcodes
        {
            for Instruction {
                attrs,
                funct3,
                itype,
                name,
                formats,
            } in instructions
            {
                let funct3 = funct3.unwrap_or(parse_quote!(0));

                let name_str = name.to_string();
                let name_ident: Ident = parse_str(&name_str.to_uppercase())?;

                let formats = formats
                    .as_ref()
                    .map(|f| f.iter().collect::<Vec<_>>())
                    .unwrap_or(itype.default_format().iter().collect::<Vec<_>>())
                    .iter()
                    .map(|f| quote!(OperandType::#f))
                    .collect::<Vec<_>>();

                tokens.extend(quote! {
                    #(#attrs_opcode)*
                    #(#attrs)*
                    pub static #name_ident: &Instruction = &Instruction {
                        name: #name_str,
                        opcode: #opcode,
                        funct3: #funct3,
                        itype: InstructionType::#itype,
                        format: &[#(#formats),*],
                    };
                });
            }
        }

        Ok(tokens)
    }

    fn generate_name_matcher(self) -> Result<TokenStream> {
        let mut arms = TokenStream::new();

        for Opcode {
            attrs: attrs_opcode,
            opcode: _,
            instructions,
        } in self.opcodes
        {
            for Instruction {
                attrs,
                funct3: _,
                itype: _,
                name,
                formats: _,
            } in instructions
            {
                let name_str = name.to_string();
                let name_ident: Ident = parse_str(&name_str.to_uppercase())?;

                arms.extend(quote! {
                    #(#attrs_opcode)*
                    #(#attrs)*
                    #name_str => Some(#name_ident),
                });
            }
        }

        Ok(quote! {
            pub fn get_by_name(name: &str) -> Option<&'static Instruction> {
                match name {
                    #arms
                    _ => None,
                }
            }
        })
    }

    fn generate_opcode_matcher(self) -> Result<TokenStream> {
        let mut arms = TokenStream::new();

        for Opcode {
            attrs: attrs_opcode,
            opcode,
            instructions,
        } in self.opcodes
        {
            for Instruction {
                attrs,
                funct3,
                itype: _,
                name,
                formats: _,
            } in instructions
            {
                let funct3 = funct3.map(|f| f.into_token_stream()).unwrap_or(quote!(..));

                let name_str = name.to_string();
                let name_ident: Ident = parse_str(&name_str.to_uppercase())?;

                arms.extend(quote! {
                    #(#attrs_opcode)*
                    #(#attrs)*
                    (#opcode, #funct3) => Some(#name_ident),
                });
            }
        }

        Ok(quote! {
            pub fn get_by_opcode(opcode: u32, funct3: u32) -> Option<&'static Instruction> {
                match (opcode, funct3) {
                    #arms
                    _ => None,
                }
            }
        })
    }
}
