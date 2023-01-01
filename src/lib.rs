use proc_macro::TokenStream;
use quote::{ quote, format_ident };
use syn::{ parse_macro_input, Data, DeriveInput };

#[proc_macro_derive(SerdeBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, vis, .. } = parse_macro_input!(input);
    let (mut field_names, mut types) = (vec![], vec![]);
    let ident_builder = format_ident!("{}Builder", ident);

    let Data::Struct(s) = data else {
        panic!("SerdeBuilder only works for structs.");
    };

    for f in s.fields {
        field_names.push(f.ident.unwrap());
        types.push(f.ty);
    }

    quote! {
        #vis struct #ident_builder {
            // TODO: if type is already `Option<_>` don't manipulate it
            #(pub #field_names: Option<#types>),*
        }

        impl #ident_builder {
            pub fn build(self) -> #ident {
                #ident {
                    // TODO: implement different build instructions for nested builder structs
                    #(#field_names: self.#field_names.unwrap_or(#ident::default().#field_names)),*
                }
            }
        }
    }.into()
}
