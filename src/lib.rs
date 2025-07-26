use convert_case::{Case, Casing};
use enum_unit_core::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

#[proc_macro_derive(EnumUnit)]
pub fn into_unit_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let old_enum_name = input.ident.clone();
    let new_enum_name = format_ident!("{}Unit", old_enum_name);

    enum InputKind {
        Struct(Vec<Ident>),
        Enum(Vec<(Ident, Fields)>),
    }

    let kind = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields_named) => {
                if fields_named.named.is_empty() {
                    return quote! {}.into();
                }
                let names = fields_named
                    .named
                    .into_iter()
                    .filter_map(|f| f.ident)
                    .map(|ident| format_ident!("{}", ident.to_string().to_case(Case::Pascal)))
                    .collect();
                InputKind::Struct(names)
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.is_empty() {
                    return quote! {}.into();
                }
                let names = (0..fields.unnamed.len())
                    .map(|i| format_ident!("{}{}", prefix(), i))
                    .collect();
                InputKind::Struct(names)
            }
            Fields::Unit => return quote! {}.into(),
        },
        Data::Enum(data) => {
            if data.variants.is_empty() {
                return quote! {}.into();
            }
            let variants = data
                .variants
                .into_iter()
                .map(|v| (v.ident, v.fields))
                .collect();
            InputKind::Enum(variants)
        }
        Data::Union(..) => return quote! { compile_error!("Unions are not supported.") }.into(),
    };

    let doc_comment = format!("Automatically generated unit-variants of [`{old_enum_name}`].");

    // Trait derivation
    let derive_inner = quote! {
        Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord
    };

    #[cfg(feature = "serde")]
    let derive_inner = quote! {
        #derive_inner, ::serde::Serialize, ::serde::Deserialize
    };

    // Collect variant names regardless of origin
    let variant_idents: Vec<Ident> = match &kind {
        InputKind::Struct(fields) => fields.clone(),
        InputKind::Enum(variants) => variants.iter().map(|(ident, _)| ident.clone()).collect(),
    };

    #[cfg(feature = "bitflags")]
    let new_enum = {
        let size = match variant_idents.len() {
            1..=8 => quote! { u8 },
            9..=16 => quote! { u16 },
            17..=32 => quote! { u32 },
            33..=64 => quote! { u64 },
            65..=128 => quote! { u128 },
            _ => {
                return quote! { compile_error!("Too many fields or variants for bitflags."); }
                    .into();
            }
        };

        let flag_consts = variant_idents.iter().enumerate().map(|(i, ident)| {
            quote! {
                const #ident = 1 << #i;
            }
        });

        quote! {
            ::bitflags::bitflags! {
                #[doc = #doc_comment]
                #[derive(#derive_inner)]
                pub struct #new_enum_name: #size {
                    #(#flag_consts)*
                }
            }
        }
    };

    #[cfg(not(feature = "bitflags"))]
    let new_enum = {
        let variants = variant_idents.iter().map(|ident| quote! { #ident, });
        quote! {
            #[doc = #doc_comment]
            #[derive(#derive_inner)]
            pub enum #new_enum_name {
                #(#variants)*
            }
        }
    };

    // Only generate kind() and From<> if the original was an enum
    let new_enum_impl = match kind {
        InputKind::Enum(ref variants) => {
            let match_arms = variants.iter().map(|(ident, fields)| match fields {
                Fields::Named(_) => quote! {
                    Self::#ident { .. } => #new_enum_name::#ident,
                },
                Fields::Unnamed(_) => quote! {
                    Self::#ident(..) => #new_enum_name::#ident,
                },
                Fields::Unit => quote! {
                    Self::#ident => #new_enum_name::#ident,
                },
            });

            let doc_comment = format!("The [`{new_enum_name}`] of this [`{old_enum_name}`].");
            quote! {
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
            }
        }
        _ => quote! {},
    };

    quote! {
        #new_enum
        #new_enum_impl
    }
    .into()
}
