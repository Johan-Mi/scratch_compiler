use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SerializeNext)]
pub fn derive_serialize_next(item: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(item as DeriveInput);
    let ident = &derive_input.ident;

    (quote! {
        impl SerializeNext for #ident {
            fn serialize_next<S>(&self, obj: &mut S) -> Result<(), S::Error>
            where
                S: SerializeMap,
            {
                obj.serialize_entry("next", &self.next.as_ref().map(|b| b.id()))?;
                Ok(())
            }
        }
    })
    .into()
}
