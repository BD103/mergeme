mod utils;

use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Data, DeriveInput, Error, Field, Fields, Meta, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

/// Automatically implements `Merge` for a given structure.
///
/// This derive will create a near-identical partial version of the structure with each field being
/// optional. For example:
///
/// ```
/// # use mergeme_derive::Merge;
/// #
/// // Deriving `Merge` on this struct...
/// #[derive(Merge)]
/// #[partial(PartialConfig)]
/// struct Config {
///     name: String,
///     version: u32,
///     dependencies: Vec<String>,
/// }
/// ```
///
/// ```
/// // ...will create this struct as well.
/// struct PartialConfig {
///     name: Option<String>,
///     version: Option<u32>,
///     dependencies: Option<Vec<String>>,
/// }
/// ```
///
/// # Attributes
///
/// - `#[partial(Name)]`, `#[partial(Name, ...)]` (struct)
///
///   *What*: This specifies the name of the partial struct generated, as well as any attributes
///   that should be applied to the partial struct.
///
///   *Where*: This should annotate the struct itself.
///
///   *How*: The name should be a single identifier inside the parenthesis, and is commonly
///   prefixed with "Partial". Attributes to be applied to the partial struct may optionally be
///   specified after name, separated by commas.
///
///   *Required*
///
/// - `#[partial(...)]` (field)
///
///   *What*: This specifies attributes that should annotate fields within the partial struct.
///
///   *Where*: This should annotate fields within the struct.
///
///   *How*: This accepts a comma-separated list of attributes to be applied to the partial's field
///   inside the parenthesis. At least one attribute is required.
///
///   *Optional*
///
/// - `#[strategy(overwrite | merge)]` (field)
///
///   *What*: This specifies how this field should be merged.
///
///   *Where*: This should annotate the struct's fields.
///
///   *How*: The value should either be `overwrite` or `merge` in parenthesis. `overwrite` will
///   replace the base's field with the partial's if it exists, while `merge` will use the field
///   type's `Merge` implementation to combine the two values together.
///
///   *Optional*: Fields without this attribute default to `overwrite`.
///
/// # Examples
///
/// ```
/// # use mergeme_derive::Merge;
/// #
/// #[derive(Merge)]
/// // Name the partial version of this type `PartialConfig`.
/// #[partial(PartialConfig)]
/// struct Config {
///     // This field will be overwritten when merged. `#[strategy(overwrite)]` may be omitted.
///     #[strategy(overwrite)]
///     name: String,
///
///     // This field will also be overwritten when merged.
///     version: u32,
///
///     // This field will be combined when merged.
///     #[strategy(merge)]
///     dependencies: Vec<String>,
/// }
/// ```
///
/// Struct and field attributes can be applied to the partial struct using the `#[partial(...)]`
/// attribute. This is commonly used to implement `Default` for the partial struct, as its fields
/// are all `Option<T>`s.
///
/// ```
/// # use mergeme_derive::Merge;
/// #
/// #[derive(Merge)]
/// // Implement `Default` for the partial struct.
/// #[partial(PartialFish, derive(Default))]
/// struct Fish {
///     fins: u8,
///     memory: f32,
///     leashed: bool,
/// }
/// #
/// # let partial_fish = PartialFish::default();
/// #
/// # assert!(partial_fish.fins.is_none());
/// # assert!(partial_fish.memory.is_none());
/// # assert!(partial_fish.leashed.is_none());
/// ```
///
/// ```
/// # use mergeme_derive::Merge;
/// # use serde::Deserialize;
/// #
/// #[derive(Merge)]
/// // Implement `Deserialize` for the partial struct, denying unknown fields.
/// #[partial(PartialConfig, derive(Deserialize), serde(deny_unknown_fields))]
/// struct Config {
///     name: String,
///
///     // When deserializing the partial struct, accept the value of either "version" or "v" for
///     // this field.
///     #[partial(serde(alias = "v"))]
///     version: u32,
/// }
/// ```
///
/// Be warned that the fields of partial structs are all `Option<T>`s. This may make certain
/// attributes like `#[serde(default)]` behave differently.
///
/// ```
/// # use mergeme::Merge;
/// # use serde::Deserialize;
/// #
/// #[derive(Merge)]
/// #[partial(PartialTrickyDefault, derive(Deserialize))]
/// struct TrickyDefault {
///     // This attribute actually gets applied to a field like `value: Option<u64>`. Because of
///     // this, the default value will be `None` and not 0.
///     #[partial(serde(default))]
///     tricky_value: u64,
///
///     // This fixes the issue by making the partial field default to `Some(0)`. Much better!
///     #[partial(serde(default = "zero_default"))]
///     corrected_value: u64,
/// }
///
/// fn zero_default() -> Option<u64> {
///     Some(0)
/// }
/// #
/// # use std::collections::HashMap;
/// # use serde::de::{IntoDeserializer, value::{Error, MapDeserializer}};
/// #
/// # // Deserialize an empty hash map, forcing the default values to be used.
/// # let map: HashMap<&'static str, u64> = HashMap::new();
/// # let deserializer: MapDeserializer<'_, _, Error> = map.into_deserializer();
/// # let partial_tricky = PartialTrickyDefault::deserialize(deserializer).unwrap();
/// #
/// # assert!(partial_tricky.tricky_value.is_none());
/// # assert_eq!(partial_tricky.corrected_value, Some(0));
/// ```
///
/// Simple generics are supported, however only generic types that can merge with themselves can
/// be annotated with `#[strategy(merge)]`.
///
/// ```
/// # use mergeme_derive::Merge;
/// #
/// #[derive(Merge)]
/// #[partial(PartialNamedData)]
/// struct NamedData<T> {
///     name: String,
///     data: T,
/// }
/// ```
///
/// ```
/// # use mergeme::Merge;
/// #
/// #[derive(Merge)]
/// #[partial(PartialNamedData)]
/// // `T: Merge<T>` means any type that can be merged with itself.
/// struct NamedData<T: Merge<T>>
/// {
///     name: String,
///     #[strategy(merge)]
///     data: T,
/// }
/// ```
///
/// Unit structs can also derive `Merge`, however there is little point in doing so.
///
/// ```
/// # use mergeme_derive::Merge;
/// #
/// #[derive(Merge)]
/// #[partial(PartialConfig)]
/// struct Config;
/// ```
///
/// # Errors
///
/// This macro only works on named structs. Enums, unions, or tuple structs will not compile.
///
/// ```compile_fail
/// # use mergeme_derive::Merge;
/// #
/// #[derive(Merge)]
/// #[partial(PartialChoice)]
/// enum Choice {
///     A,
///     B,
/// }
/// ```
///
/// ```compile_fail
/// # use mergeme_derive::Merge;
/// #
/// #[derive(Merge)]
/// #[partial(PartialConfig)]
/// struct Config(bool, u8, Vec<String>);
/// ```
///
/// This macro requires a single `#[partial(...)]` attribute on the struct itself.
///
/// ```compile_fail
/// # use mergeme_derive::Merge;
/// #
/// #[derive(Merge)]
/// // Missing `#[partial(...)]`.
/// struct Config {
///     name: String,
///     dependencies: Vec<String>,
/// }
/// ```
///
/// ```compile_fail
/// # use mergeme_derive::Merge;
/// #
/// #[derive(Merge)]
/// // Too many `#[partial(...)]`s.
/// #[partial(PartialConfig1)]
/// #[partial(PartialConfig2)]
/// struct Config {
///     name: String,
///     dependencies: Vec<String>,
/// }
/// ```
///
/// This macro only supports the `overwrite` and `merge` strategies.
///
/// ```compile_fail
/// # use mergeme_derive::Merge;
/// #
/// #[derive(Merge)]
/// #[partial(PartialDog)]
/// struct Dog {
///     name: String,
///     // `add` is not a valid strategy.
///     #[strategy(add)]
///     age: u16,
/// }
/// ```
#[proc_macro_derive(Merge, attributes(partial, strategy))]
pub fn derive_merge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_merge_inner(input) {
        Ok(stream) => stream.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// The implementation of `#[derive(Merge)]`.
fn derive_merge_inner(input: DeriveInput) -> Result<TokenStream> {
    let struct_name = &input.ident;
    let struct_vis = &input.vis;
    let struct_generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();

    let struct_fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        Data::Enum(_) => {
            return Err(Error::new_spanned(
                input,
                "`#[derive(Merge)]` only works on structs, not enums",
            ));
        }
        Data::Union(_) => {
            return Err(Error::new_spanned(
                input,
                "`#[derive(Merge)]` only works on structs, not unions",
            ));
        }
    };

    let (partial_name, partial_meta) =
        partial_name_and_meta(&input).map(|(name, meta)| (name, meta.into_iter()))?;

    let partial_fields = partial_fields(struct_fields)?;

    let merge_in_place = merge_in_place(struct_fields)?;

    let output = quote! {
        impl #impl_generics ::mergeme::Merge<#partial_name #ty_generics> for #struct_name #ty_generics #where_clause {
            fn merge_in_place(&mut self, other: #partial_name #ty_generics) {
                #merge_in_place
            }
        }

        #(#[#partial_meta])*
        #struct_vis struct #partial_name #struct_generics #where_clause {
            #partial_fields
        }
    };

    Ok(output)
}

fn partial_name_and_meta(input: &DeriveInput) -> Result<(Ident, Punctuated<Meta, Token![,]>)> {
    let mut name: Option<Ident> = None;
    let mut meta: Punctuated<Meta, Token![,]> = Punctuated::new();

    for attr in input.attrs.iter() {
        if attr.path().is_ident("partial") {
            attr.parse_args_with(|input: ParseStream<'_>| {
                if name.is_some() {
                    return Err(Error::new_spanned(
                        attr,
                        "multiple `#[partial(...)]` attributes on the struct is disallowed",
                    ));
                }

                name = Some(input.parse()?);

                if input.parse::<Token![,]>().is_ok() {
                    let punctuated = input.parse_terminated(Meta::parse, Token![,])?;
                    meta.extend(punctuated.into_pairs());
                }

                Ok(())
            })?;
        }
    }

    match name {
        Some(name) => Ok((name, meta)),
        None => Err(Error::new_spanned(
            utils::DeriveInputWithoutData(input),
            "expected `#[partial(...)]`",
        )),
    }
}

fn partial_fields(fields: &Fields) -> Result<TokenStream> {
    let mut stream = TokenStream::new();

    for field in fields {
        let Field {
            attrs,
            vis,
            mutability: _,
            ident,
            colon_token,
            ty,
        } = field;

        let mut field_meta: Punctuated<Meta, Token![,]> = Punctuated::new();

        for attr in attrs {
            if attr.path().is_ident("partial") {
                attr.parse_args_with(|input: ParseStream<'_>| {
                    let punctuated = input.parse_terminated(Meta::parse, Token![,])?;
                    field_meta.extend(punctuated);

                    Ok(())
                })?;
            }
        }

        let field_meta = field_meta.into_iter();

        let partial_ty = quote_spanned!(ty.span()=> ::core::option::Option<#ty>);

        let field = quote_spanned! {field.span()=>
            #(#[#field_meta])*
            #vis #ident #colon_token #partial_ty,
        };

        stream.extend(field);
    }

    Ok(stream)
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
