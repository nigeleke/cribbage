mod derive_id;

// ------------------------------------
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(IdV4)]
pub fn derive_id_v4(input: TokenStream) -> TokenStream {
    derive_id::derive_id(input, quote!(::uuid::Uuid::new_v4))
}

#[proc_macro_derive(IdV7)]
pub fn derive_id_v7(input: TokenStream) -> TokenStream {
    derive_id::derive_id(input, quote!(::uuid::Uuid::now_v7))
}
