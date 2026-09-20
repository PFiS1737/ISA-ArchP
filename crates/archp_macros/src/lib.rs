mod instructions;

use syn::parse_macro_input;

#[proc_macro]
pub fn instructions(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let root = parse_macro_input!(input as instructions::Root);

    match root.generate() {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
