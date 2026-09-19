use std::{fs, path::Path};

use anyhow::Result;
use archp_assembler::types::{InstructionType, OperandType};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use syn::{Expr, Ident};

use crate::common::generated_header;

#[derive(Serialize, Deserialize, JsonSchema)]
struct Root {
    opcodes: Vec<Opcode>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct Opcode {
    feature: Option<String>,
    opcode: u32,
    #[serde(flatten)]
    instructions: Funct3OrInstruction,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
enum Funct3OrInstruction {
    Funct3 { funct3s: Vec<Funct3> },
    Instruction { instruction: Instruction },
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct Funct3 {
    funct3: u32,
    instruction: Instruction,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct Instruction {
    name: String,
    #[serde(rename = "type")]
    itype: InstructionType,
    format: Option<Vec<OperandType>>,
}

pub fn generate<P1: AsRef<Path>, P2: AsRef<Path>>(src_file: P1, out_dir: P2) -> Result<()> {
    let schema = schemars::schema_for!(Root);
    let json = serde_json::to_string_pretty(&schema)?;

    let dest_file_schema = out_dir.as_ref().join("instructions.schema.json");
    fs::write(&dest_file_schema, json)?;
    eprintln!(
        "Generated schema for instructions at {}",
        dest_file_schema.display()
    );

    let content = fs::read_to_string(src_file)?;
    let root: Root = toml::from_str(&content)?;

    let tokens = root.generate()?;
    let code = prettyplease::unparse(&syn::parse2(tokens)?);

    let dest_file_code = out_dir.as_ref().join("instructions.rs");
    fs::write(&dest_file_code, code)?;
    eprintln!(
        "Generated instructions code at {}",
        dest_file_code.display()
    );

    Ok(())
}

impl Root {
    fn generate(self) -> Result<TokenStream> {
        let mut out = generated_header();

        out.extend(quote! {
            use crate::instructions::Instruction;
            use crate::types::{InstructionType, OperandType};
        });

        out.extend(self.generate_definitions()?);

        out.extend(self.generate_name_matcher()?);

        out.extend(self.generate_opcode_matcher()?);

        Ok(out)
    }

    fn instructions(&self) -> impl Iterator<Item = (u32, u32, &Instruction, &Option<String>)> {
        self.opcodes.iter().flat_map(|o| match &o.instructions {
            Funct3OrInstruction::Funct3 { funct3s } => funct3s
                .iter()
                .map(|f| (o.opcode, f.funct3, &f.instruction, &o.feature))
                .collect(),
            Funct3OrInstruction::Instruction { instruction } => {
                vec![(o.opcode, 0, instruction, &o.feature)]
            },
        })
    }

    fn generate_definitions(&self) -> Result<TokenStream> {
        self.instructions()
            .map(
                |(opcode, funct3, instruction, feature)| -> Result<TokenStream> {
                    let name = &instruction.name;

                    let name_ident: Ident = syn::parse_str(&name.to_uppercase())?;
                    let itype_ident: Ident = syn::parse_str(&instruction.itype.to_string())?;

                    let format = instruction
                        .format
                        .as_ref()
                        .unwrap_or(&instruction.itype.default_format().to_vec())
                        .iter()
                        .map(|o| -> Result<TokenStream> {
                            let ident: Expr = syn::parse_str(&o.to_string())?;
                            Ok(quote! { OperandType::#ident })
                        })
                        .collect::<Result<Vec<_>>>()?;

                    let feature_attr = feature.as_ref().map(|f| {
                        quote! { #[cfg(feature = #f)] }
                    });

                    Ok(quote! {
                        #feature_attr
                        pub static #name_ident: &Instruction = &Instruction {
                            name: #name,
                            opcode: #opcode,
                            funct3: #funct3,
                            itype: InstructionType::#itype_ident,
                            format: &[#(#format),*],
                        };
                    })
                },
            )
            .collect::<Result<TokenStream>>()
    }

    fn generate_name_matcher(&self) -> Result<TokenStream> {
        let match_arms = self
            .instructions()
            .map(|(_, _, instruction, feature)| {
                let name = &instruction.name;
                let name_ident: Ident = syn::parse_str(&name.to_uppercase())?;

                let feature_attr = feature.as_ref().map(|f| {
                    quote! { #[cfg(feature = #f)] }
                });

                Ok(quote! {
                    #feature_attr
                    #name => Some(#name_ident),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            pub fn get_by_name(name: &str) -> Option<&'static Instruction> {
                match name {
                    #(#match_arms)*
                    _ => None,
                }
            }
        })
    }

    fn generate_opcode_matcher(&self) -> Result<TokenStream> {
        let match_arms = self
            .instructions()
            .map(|(opcode, funct3, instruction, feature)| {
                let name_ident: Ident = syn::parse_str(&instruction.name.to_uppercase())?;

                let feature_attr = feature.as_ref().map(|f| {
                    quote! { #[cfg(feature = #f)] }
                });

                Ok(quote! {
                    #feature_attr
                    (#opcode, #funct3) => Some(#name_ident),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            pub fn get_by_opcode(opcode: u32, funct3: u32) -> Option<&'static Instruction> {
                match (opcode, funct3) {
                    #(#match_arms)*
                    _ => None,
                }
            }
        })
    }
}
