use proc_macro::TokenStream;
use quote::{ quote, format_ident };

use syn::{
    parse_macro_input,
    Attribute,
    Data,
    DeriveInput,
    Ident,
};

trait EqStr {
    fn eq_str(&self, s: &str) -> bool;
}

impl EqStr for Ident {
    fn eq_str(&self, s: &str) -> bool {
        *self == Self::new(s, self.span())
    }
}

#[proc_macro_derive(SerdeBuilder, attributes(builder_derive))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, vis, attrs, .. } = parse_macro_input!(input);
    let (mut field_names, mut types) = (vec![], vec![]);
    let mut derives = quote!();
    let ident_builder = format_ident!("{}Builder", ident);

    let Data::Struct(s) = data else {
        panic!("SerdeBuilder only works for structs.");
    };

    for Attribute { path, tokens, .. } in attrs {
        let ident = path.get_ident().expect("Unexpected attribute.");

        if ident.eq_str("builder_derive") {
            derives = quote! { #[derive #tokens] };
            break
        }
    }

    for f in s.fields {
        field_names.push(f.ident.unwrap());
        types.push(f.ty);
    }

    quote! {
        #derives
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
