mod utils;

use {
    utils::EqStr,
    proc_macro::TokenStream,

    quote::{
        format_ident,
        quote,
    },

    syn::{
        parse_macro_input,
        Attribute,
        Data,
        DeriveInput,
        Field,
        Type,
    }
};

#[proc_macro_derive(SerdeBuilder, attributes(builder_derive, use_builder))]
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

    for Field { ident: f_ident, attrs: f_attrs, ty, .. } in s.fields {
        let Type::Path(f_type) = ty else {
            panic!("Type not supported");
        };

        let mut is_builder = false;
        let mut f_type_builder = f_type.clone();

        if let Some(f_type_builder_last_seg) = f_type_builder.path.segments.last_mut() {
            f_type_builder_last_seg.ident = format_ident!("{}Builder", f_type_builder_last_seg.ident);
        }

        for Attribute { path, tokens, .. } in f_attrs {
            if let Some(a_ident) = path.get_ident() {
                if a_ident.eq_str("use_builder") {
                    if tokens.is_empty() {
                        is_builder = true;
                        break
                    } else {
                        panic!("The `use_builder` attribute takes no arguments.");
                    }
                }
            }
        }

        let Some(f_type_last_seg) = f_type.path.segments.last() else {
            panic!("Unable to parse type");
        };

        let [new_type, build_ins] = if is_builder {[
            quote! { Option<#f_type_builder> },
            quote! {
                match self.#f_ident {
                    Some(builder) => builder.build(),
                    None => #f_type::default()
                }
            }
        ]} else if f_type_last_seg.ident.eq_str("Option") {[
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
