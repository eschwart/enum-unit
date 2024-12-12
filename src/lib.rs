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

    // Create the variants for the new enum (copied from the original)
    let flag_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! { #ident, }
    });

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

    quote! {
        #[doc = #doc_comment]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum #new_enum_name {
            #(#flag_arms)*
        }

        impl From<#old_enum_name> for #new_enum_name {
            fn from(value: #old_enum_name) -> Self {
                match value {
                    #(#match_arms)*
                }
            }
        }
    }
    .into()
}
