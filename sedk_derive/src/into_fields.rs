use proc_macro2::{TokenStream, Ident, Span};
use quote::{quote};
use syn::{Data, DataStruct, DeriveInput, Field, Fields, punctuated::Punctuated};


const BREAK_GLASS: &'static str = "IntoFields can only be derived for structs with named fields.";

pub fn impl_into_fields(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;
    let struct_fields = match input.data {
        // copied from https://blog.turbo.fish/proc-macro-simple-derive/
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!(BREAK_GLASS),
    };
    let name_ident = Ident::new("s", Span::call_site());

    let without_name = field_mappings(&struct_fields, None); // we need the "code" for both branches.
    let with_name = field_mappings(&struct_fields, Some(&name_ident));


    quote! {
        #[automatically_derived]
        impl IntoFields for #struct_name {
            fn into_fields(name: Option<&str>) -> Vec<Field> {
                let fields: Vec<Vec<Field>> = match name {
                    None => #without_name,
                    Some(#name_ident) => #with_name
                };
                fields.into_iter().flatten().collect()
            }
        }
    }
}

fn field_mappings<T>(struct_fields: &Punctuated<Field, T>, with_name: Option<&Ident>) -> TokenStream {
    let field_mappings = struct_fields.iter().map(|field| {
        let field_name = match (&field.ident, with_name) {
            (None, _) => panic!(BREAK_GLASS),
            (Some(ident), None) => {
                let as_str = ident.to_string();
                quote! {Some(#as_str)}
            },
            (Some(ident), Some(s)) => {
                let as_str = ident.to_string();
                quote! {
                    Some(&(#s.to_owned() + "." + #as_str))
                }
            }
        };
        let field_type = &field.ty;
        quote! {
            #field_type::into_fields(#field_name)
        }
    });
    return quote! {vec![#(#field_mappings),*]}
}
