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
        GenericParam,
        Type,
    }
};

#[proc_macro_derive(SerdeBuilder, attributes(builder_derive, use_builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, vis, attrs, generics, .. } = parse_macro_input!(input);
    let (mut field_names, mut types, mut build_instructions) = (vec![], vec![], vec![]);
    let (mut derives, mut where_clause, mut generic_type_annotations) = (quote!(), quote!(), quote!());
    let ident_builder = format_ident!("{}Builder", ident);

    let Data::Struct(s) = data else {
        panic!("SerdeBuilder only works for structs.");
    };

    if generics.params.len() > 0 {
        let generic_params = generics.params.clone();

        let generic_types: Vec<_> = generic_params
            .iter()
            .filter(|p| matches!(p, GenericParam::Type(_)))
            .collect();

        generic_type_annotations = quote! { #generic_params };
        where_clause = quote! { where #(#generic_types: Default),* };
    }

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
            quote! { self.#f_ident.unwrap_or(#ident::<#generic_type_annotations>::default().#f_ident) }
        ]};

        types.push(new_type);
        field_names.push(f_ident.expect("Unable to read field identifier"));
        build_instructions.push(build_ins);
    }

    quote! {
        #derives
        #vis struct #ident_builder #generics {
            #(pub #field_names: #types),*
        }

        impl #generics #ident_builder #generics #where_clause {
            pub fn build(self) -> #ident #generics {
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
