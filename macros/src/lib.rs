use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(crab_worm)]
pub fn crab_worm_derive(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let type_name = ast.ident.to_string();

    let metadata = match &ast.data {
        Data::Struct(data) => {
            let fields = match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => panic!("Crab-Worm only supports named fields"),
            };

            let names: Vec<String> = fields
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect();
            let types: Vec<String> = fields
                .iter()
                .map(|f| f.ty.to_token_stream().to_string())
                .collect();

            quote! {
                ::crab_worm_core::meta::TypeMetadata::Struct(
                    ::crab_worm_core::meta::StructMetadata::new(
                        #type_name,
                        &[ #( ::crab_worm_core::meta::FieldMetadata::new(#names, #types) ),* ],
                    )
                )
            }
        }
        Data::Enum(data) => {
            let variants: Vec<String> = data
                .variants
                .iter()
                .map(|v| match v.fields {
                    Fields::Unit => v.ident.to_string(),
                    _ => panic!("Crab-Worm only supports unit enums (variants without data)"),
                })
                .collect();

            quote! {
                ::crab_worm_core::meta::TypeMetadata::Enum(
                    ::crab_worm_core::meta::EnumMetadata::new(
                        #type_name,
                        &[ #(#variants),* ],
                    )
                )
            }
        }
        Data::Union(_) => panic!("Crab-Worm only supports structs and enums"),
    };

    let expanded = quote! {
        ::crab_worm_core::inventory::submit! { #metadata }
    };

    expanded.into()
}