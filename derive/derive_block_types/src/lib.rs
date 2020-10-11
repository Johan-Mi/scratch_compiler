use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SimpleHatBlock)]
pub fn derive_simple_hat_block(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let ident = &derive_input.ident;

    (quote! {
        impl SimpleHatBlock for #ident {
            const OPCODE: &'static str = Self::OPCODE;

            fn get_id(&self) -> &ID {
                &self.id
            }

            fn get_next(&self) -> &Option<Box<Block>> {
                &self.next
            }
        }
    })
    .into()
}
