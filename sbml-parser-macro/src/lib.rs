use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(XmlFlatten)]
pub fn xml_flatten_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Generate field names and types from the struct
    let fields = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let field_type = &f.ty;
                quote! {
                    let #field_name = map.remove(stringify!(#field_name))
                        .ok_or_else(|| serde::de::Error::missing_field(stringify!(#field_name)))?
                        .deserialize_into::<#field_type>()?;
                }
            }).collect::<Vec<_>>()
        } else {
            panic!("XmlFlatten only supports structs with named fields.");
        }
    } else {
        panic!("XmlFlatten only supports structs.");
    };

    // Generate the final struct construction
    let struct_fields = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    quote! { #field_name }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let expanded = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Error;
                use quick_xml::de::from_str;
                use std::collections::HashMap;

                #[derive(serde::Deserialize)]
                struct TempMap(HashMap<String, String>);

                let map: HashMap<String, String> = TempMap::deserialize(deserializer)?.0;

                #(#fields)*

                Ok(#name {
                    #(#struct_fields),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
