use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Data, DeriveInput};

/// A wrapper around [`DeriveInput`] whose [`ToTokens`] implementation does not emit the [`Data`]
/// tokens.
///
/// This is intended for error messages that do not want to span all fields of a struct.
pub struct DeriveInputWithoutData<'a>(pub &'a DeriveInput);

impl ToTokens for DeriveInputWithoutData<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let input = self.0;

        for attr in input.attrs.iter() {
            attr.to_tokens(tokens);
        }

        input.vis.to_tokens(tokens);

        match input.data {
            Data::Struct(ref d) => d.struct_token.to_tokens(tokens),
            Data::Enum(ref d) => d.enum_token.to_tokens(tokens),
            Data::Union(ref d) => d.union_token.to_tokens(tokens),
        }

        input.ident.to_tokens(tokens);

        input.generics.to_tokens(tokens);
    }
}
