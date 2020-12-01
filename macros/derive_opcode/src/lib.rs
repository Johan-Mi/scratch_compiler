use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};

#[proc_macro_attribute]
pub fn opcode(attr: TokenStream, item: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(attr as AttributeArgs);
    let ident = &derive_input.ident;
    assert!(args.len() == 1);
    let opcode_name = &args[0];

    (quote! {
        #derive_input

        impl Opcode for #ident {
            const OPCODE: &'static str = #opcode_name;
        }
    })
    .into()
}
