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
        // return empty if not an enum
        return quote! { compile_error!("Unsupported structure (enum's only)") }.into();
    };

    let old_enum_name = input.ident;
    let new_enum_name = quote::format_ident!("{}Unit", old_enum_name);

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

    let doc_comment = format!(
        "Automatically generated unit-variants of [`{}`].",
        old_enum_name
    );

    #[cfg(not(feature = "bitflag"))]
    let new_enum = {
        let flag_arms = variants.iter().map(|variant| {
            let ident = &variant.ident;
            quote! { #ident, }
        });

        quote! {
            #[doc = #doc_comment]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub enum #new_enum_name {
                #(#flag_arms)*
            }
        }
    };

    #[cfg(feature = "bitflag")]
    let new_enum = {
        let n = variants.len();

        let size = if n <= 8 {
            quote! { u8 }
        } else if n <= 16 {
            quote! { u16 }
        } else if n <= 32 {
            quote! { u32 }
        } else if n <= 64 {
            quote! { u64 }
        } else if n <= 128 {
            quote! { u128 }
        } else {
            return quote! { compile_error!("Enum has too many variants."); }.into();
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
                #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
                pub struct #new_enum_name: #size {
                    #(#flag_arms)*
                }
            }
        }
    };

    let doc_comment = format!("The [`{}`] of this [`{}`].", new_enum_name, old_enum_name);

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

    quote! {
        #new_enum
        #new_enum_impl
    }
    .into()
}
