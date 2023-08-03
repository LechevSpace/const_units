use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;
use syn::{Data, Ident};

pub(crate) fn generate_derive_adding_mul(item: Ident, data: Data, path: Path) -> TokenStream {
    match data {
        Data::Struct(struct_data) => {
            let fields_expanded = struct_data.fields.iter().enumerate().map(|(idx, field)| {
                match field.ident.clone() {
                    Some(ident) => ident,
                    None => Ident::new(&(idx as u32).to_string(), Span::call_site()),
                }
            });
            quote!(
                impl const const_ops::Mul for #item {
                    type Output = Self;

                    fn mul(self, rhs: Self) -> Self::Output {
                        Self{ #(#fields_expanded: #path::Add::add(self. #fields_expanded, rhs. #fields_expanded)),*}
                    }
                }

            )
        }
        Data::Enum(_) => panic!("not available for enums"),
        Data::Union(_) => panic!("not available for unions"),
    }
}
