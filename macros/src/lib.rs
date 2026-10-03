use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(crab_worm)]
pub fn crab_worm_derive(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_str = ast.ident.to_string();

    let extracted_fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Crab-Worm only supports named fields"),
        },
        _ => panic!("Crab-Worm only supports structs"),
    };

    let names: Vec<String> = extracted_fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();
    let types: Vec<String> = extracted_fields
        .iter()
        .map(|f| f.ty.to_token_stream().to_string())
        .collect();

    let expanded = quote! {
        ::crab_worm_core::inventory::submit! {
            ::crab_worm_core::meta::StructMetadata::new(
                #struct_str,
                &[ #( ::crab_worm_core::meta::FieldMetadata::new(#names, #types) ),* ],
            )
        }
    };

    expanded.into()
}