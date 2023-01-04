use syn::Ident;

pub trait EqStr {
    fn eq_str(&self, s: &str) -> bool;
}

impl EqStr for Ident {
    fn eq_str(&self, s: &str) -> bool {
        *self == Self::new(s, self.span())
    }
}
