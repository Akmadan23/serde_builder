use proc_macro::TokenStream;
use quote::{ quote, format_ident };

use syn::{
    parse_macro_input,
    Attribute,
    Data,
    DeriveInput,
    Field,
    Ident,
    Type,
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
    let (mut field_names, mut types, mut build_instructions) = (vec![], vec![], vec![]);
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

    for Field { ident: f_ident, ty, .. } in s.fields {
        let Type::Path(f_type) = ty else {
            panic!("Type not supported");
        };

        let Some(f_type_last_seg) = f_type.path.segments.last() else {
            panic!("Unable to parse type");
        };

        let [new_type, build_ins] = if f_type_last_seg.ident.eq_str("Option") {[
            quote! { #f_type },
            quote! { self.#f_ident }
        ]} else {[
            quote! { Option<#f_type> },
            quote! { self.#f_ident.unwrap_or(#ident::default().#f_ident) }
        ]};

        types.push(new_type);
        field_names.push(f_ident.expect("Unable to read field identifier"));
        build_instructions.push(build_ins);
    }

    quote! {
        #derives
        #vis struct #ident_builder {
            #(pub #field_names: #types),*
        }

        impl #ident_builder {
            pub fn build(self) -> #ident {
                #ident {
                    // TODO: implement different build instructions for nested builder structs
                    #(#field_names: #build_instructions),*
                }
            }
        }
    }.into()
}

#[cfg(test)]
mod test {
    use trybuild::TestCases;

    #[test]
    fn trybuild_tests() {
        let t = TestCases::new();
        t.pass("tests/pass/*.rs");
        t.compile_fail("tests/fail/*.rs");
    }
}
