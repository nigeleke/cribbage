use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub(crate) fn derive_id(input: TokenStream, uuid_new: TokenStream2) -> TokenStream {
    derive_id_impl(parse_macro_input!(input as DeriveInput), uuid_new)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_id_impl(input: DeriveInput, uuid_new: TokenStream2) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let name_string = name.to_string();

    let prefix = name_string
        .strip_suffix("Id")
        .ok_or_else(|| syn::Error::new_spanned(name, "Id type name must end with `Id`"))?;

    let prefix = match prefix {
        "" => Err(syn::Error::new_spanned(
            name,
            "Id type name must have a prefix before `Id`",
        )),
        prefix => Ok(prefix),
    }?;

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(()),
            _ => Err(syn::Error::new_spanned(
                &input,
                "Id must be a tuple struct with exactly one field",
            )),
        },
        _ => Err(syn::Error::new_spanned(
            &input,
            "Id can only be derived for a tuple struct",
        )),
    }?;

    let prefix = prefix.to_ascii_lowercase();
    let display_format = format!("{prefix}-{{}}");

    Ok(quote! {
        impl #name {
            /// Create a new #name
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self(#uuid_new())
            }
        }

        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, #display_format, self.0)
            }
        }
    })
}
