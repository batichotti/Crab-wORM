use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(crab_worm)]
pub fn crab_worm_derive(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let struct_name = &ast.ident;

    let extracted_fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Crab Worm only supports named fields"),
        },
        _ => panic!("Crab Worm only supports structs"),
    };

    let fields: Vec<(String, String, String)> = extracted_fields
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap().to_string();

            let ty = &field.ty;
            let ty = quote!(#ty).to_string();

            let vis = &field.vis;
            let vis = quote!(#vis).to_string();

            (name, ty, vis)
        })
        .collect();

    let fields = fields
        .iter()
        .map(|(name, ty, vis)| quote! { (#name, #ty, #vis) });

    let expanded = quote! {
        impl #struct_name {
            pub const CRAB_WORM_FIELDS: &'static [(&'static str, &'static str, &'static str)] = &[
                #(#fields),*
            ];
        }
    };

    expanded.into()
}