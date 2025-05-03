use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Data, DataEnum, DataUnion, DeriveInput, Error, Field, Fields, Result, parse_macro_input,
    spanned::Spanned,
};

#[proc_macro_derive(Merge, attributes(partial, strategy))]
pub fn derive_merge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let struct_vis = &input.vis;
    let struct_generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();

    let struct_fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        Data::Enum(DataEnum { enum_token, .. }) => {
            return Error::new(
                enum_token.span,
                "`#[derive(Merge)]` only works on `struct`s, not `enum`s",
            )
            .to_compile_error()
            .into();
        }
        Data::Union(DataUnion { union_token, .. }) => {
            return Error::new(
                union_token.span,
                "`#[derive(Merge)]` only works on `struct`s, not `union`s",
            )
            .to_compile_error()
            .into();
        }
    };

    let partial_name = match partial_name(&input) {
        Ok(struct_config) => struct_config,
        Err(error) => return error.to_compile_error().into(),
    };

    let partial_fields = partial_fields(struct_fields);

    let merge_in_place = match merge_in_place(struct_fields) {
        Ok(merge_in_place) => merge_in_place,
        Err(error) => return error.to_compile_error().into(),
    };

    let output = quote! {
        impl #impl_generics ::mergeme::Merge<#partial_name #ty_generics> for #struct_name #ty_generics #where_clause {
            fn merge_in_place(&mut self, other: #partial_name #ty_generics) {
                #merge_in_place
            }
        }

        #struct_vis struct #partial_name #struct_generics #where_clause {
            #partial_fields
        }
    };

    output.into()
}

fn partial_name(input: &DeriveInput) -> Result<Ident> {
    let mut partial: Option<Ident> = None;

    for attr in input.attrs.iter() {
        if attr.path().is_ident("partial") {
            attr.parse_nested_meta(|meta| {
                partial = Some(meta.path.require_ident()?.clone());
                Ok(())
            })?;
        }
    }

    partial.ok_or_else(|| Error::new(input.span(), "expected `#[partial(...)]`"))
}

fn partial_fields(fields: &Fields) -> TokenStream {
    let partial_fields = fields.iter().map(|field| {
        let Field {
            attrs,
            vis,
            mutability: _,
            ident,
            colon_token,
            ty,
        } = field;

        let filtered_attrs = attrs
            .iter()
            .filter(|attr| !attr.path().is_ident("strategy"));

        let partial_ty = quote_spanned!(ty.span()=> ::core::option::Option<#ty>);

        quote_spanned! {field.span()=>
            #(#filtered_attrs)*
            #vis #ident #colon_token #partial_ty
        }
    });

    quote_spanned!(fields.span()=> #(#partial_fields),*)
}

fn merge_in_place(fields: &Fields) -> Result<TokenStream> {
    #[derive(Default)]
    enum MergeStrategy {
        #[default]
        Overwrite,
        Merge,
    }

    let merge_in_place = fields.iter().map(|field| {
        let mut strategy = MergeStrategy::default();

        for attr in field.attrs.iter() {
            if attr.path().is_ident("strategy") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("overwrite") {
                        strategy = MergeStrategy::Overwrite;
                        return Ok(());
                    }

                    if meta.path.is_ident("merge") {
                        strategy = MergeStrategy::Merge;
                        return Ok(());
                    }

                    Err(Error::new(
                        meta.path.span(),
                        "expected `#[strategy(overwrite)]` or `#[strategy(merge)]`",
                    ))
                })?;
            }
        }

        let Some(ref field_name) = field.ident else {
            return Err(Error::new(
                field.span(),
                "`#[derive(Merge)]` does not support tuple `struct`s",
            ));
        };

        let merge = match strategy {
            MergeStrategy::Overwrite => quote! {
                self.#field_name = #field_name;
            },
            MergeStrategy::Merge => quote! {
                ::mergeme::Merge::merge_in_place(&mut self.#field_name, #field_name);
            },
        };

        Ok(quote! {
            if let ::core::option::Option::Some(#field_name) = other.#field_name {
                #merge
            }
        })
    });

    let mut stream = TokenStream::new();

    for merge_field in merge_in_place {
        merge_field?.to_tokens(&mut stream);
    }

    Ok(stream)
}
