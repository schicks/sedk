use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_indexable(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;

    quote! {
        #[automatically_derived]
        impl Indexable for #struct_name {
            fn index_mapping() -> IndexMapping {
                IndexMapping {fields: #struct_name::into_fields(None)}
            }
        }
    }
}
