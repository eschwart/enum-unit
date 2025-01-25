use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(EnumUnit)]
pub fn into_unit_enum(input: TokenStream) -> TokenStream {
    // parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // check if the input is an enum
    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        return quote! { compile_error!("Unsupported structure (enum's only)") }.into();
    };

    // return nothing if there are no variants
    if variants.is_empty() {
        return quote! {}.into();
    }

    // prepare names for each enumeration
    let old_enum_name = input.ident;
    let new_enum_name = quote::format_ident!("{}Unit", old_enum_name);

    // obtain names of every variant
    let match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;

        // Handle tuple and struct variants
        match &variant.fields {
            Fields::Unit => {
                quote! {
                    #old_enum_name::#ident => #new_enum_name::#ident,
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    #old_enum_name::#ident(..) => #new_enum_name::#ident,
                }
            }
            Fields::Named(_) => {
                quote! {
                    #old_enum_name::#ident { .. } => #new_enum_name::#ident,
                }
            }
        }
    });

    // primitive derivations
    let derive_inner = quote! {
        Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord
    };

    // serde derivations
    #[cfg(feature = "serde")]
    let derive_inner = quote! {
        #derive_inner, ::serde::Serialize, ::serde::Deserialize
    };

    let doc_comment = format!(
        "Automatically generated unit-variants of [`{}`].",
        old_enum_name
    );

    // basic implementation
    #[cfg(not(feature = "bitflags"))]
    let new_enum = {
        let flag_arms = variants.iter().map(|variant| {
            let ident = &variant.ident;
            quote! { #ident, }
        });

        quote! {
            #[doc = #doc_comment]
            #[derive(#derive_inner)]
            pub enum #new_enum_name {
                #(#flag_arms)*
            }
        }
    };

    // bitflags implementation
    #[cfg(feature = "bitflags")]
    let new_enum = {
        // size of the bitflag
        let size = match variants.len() {
            1..=8 => quote! { u8 },
            9..=16 => quote! { u16 },
            17..=32 => quote! { u32 },
            33..=64 => quote! { u64 },
            65..=128 => quote! { u128 },
            _ => return quote! { compile_error!("Enum has too many variants."); }.into(),
        };

        let flag_arms = variants.iter().enumerate().map(|(i, variant)| {
            let ident = &variant.ident;
            quote! {
                const #ident = 1 << #i;
            }
        });

        quote! {
            ::bitflags::bitflags! {
                #[doc = #doc_comment]
                #[derive(#derive_inner)]
                pub struct #new_enum_name: #size {
                    #(#flag_arms)*
                }
            }
        }
    };

    let doc_comment = format!("The [`{}`] of this [`{}`].", new_enum_name, old_enum_name);

    // [`kind`] method and [`From`] trait implementation
    let new_enum_impl = quote! {
        impl #old_enum_name {
            #[doc = #doc_comment]
            pub const fn kind(&self) -> #new_enum_name {
                match self {
                    #(#match_arms)*
                }
            }
        }

        impl From<#old_enum_name> for #new_enum_name {
            fn from(value: #old_enum_name) -> Self {
                value.kind()
            }
        }
    };

    // putting it all together
    quote! {
        #new_enum
        #new_enum_impl
    }
    .into()
}
