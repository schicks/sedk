use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

pub fn impl_indexable(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;
    let struct_fields = match input.data {
        // copied from https://blog.turbo.fish/proc-macro-simple-derive/
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only structs with name fields can be converted to index mappings."),
    };

    let field_mappings = struct_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #field_type::into_fields(#(#field_name => "{}"))
        }
    });

    quote! {
        #[automatically_derived]
        impl Indexable for #struct_name {
            fn index_mapping() -> IndexMapping {
                IndexMapping {fields: vec![
                    #(#field_mappings),*
                ].iter().flatten().collect()}
            }
        }
    }
}
