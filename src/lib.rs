//! A crate providing a [Splat] derive macro, which generates a `splat` method for
//! the struct deriving it.
//!
//! # The `splat` method
//!
//! The `splat` method is commonly defined for structs with numeric fields
//! of the same type. It takes a value `v` and returns an instance of the struct
//! where each field is set to `v`.
//!
//! This crate provides a macro that generates a `splat` method for any struct that
//! has fields which are all of the same type. However, the type shared by each field 
//! must implement [Clone].

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Iter, Data, DeriveInput, Field, Fields, Type};

/// Derive macro generating a `splat` method for the struct
///
/// The macro will only work under the following conditions:
///
/// - The data structure is a struct (not an enum)
/// - The struct has at least one field
/// - Every field in the struct is of the same type
/// - The type shared by each field implements [Clone]
///
/// # Examples
///
/// ## Struct
/// ```
/// use splat_derive::Splat;
///
/// // macro used here
/// #[derive(Splat)]
/// struct Foo {
///     field_one: u8,
///     field_two: u8,
///     field_three: u8,
/// }
///
/// // generated code
/// /*
/// impl Foo {
///     fn splat(v: u8) -> Self {
///         Foo {
///             field_one: v.clone(),
///             field_two: v.clone(),
///             field_three: v.clone(),
///         }
///     }
/// }
/// */
///
/// fn bar() {
///     let foo = Foo::splat(2);
///     assert_eq!(foo.field_one, 2);
///     assert_eq!(foo.field_two, 2);
///     assert_eq!(foo.field_three, 2);
/// }
/// ```
///
/// ## Tuple Struct
/// ```
/// use splat_derive::Splat;
///
/// // macro used here
/// #[derive(Splat)]
/// struct Foo(i8, i8);
///
/// // generated code
/// /*
/// impl Foo {
///     fn splat(v: u8) -> Self {
///         Foo(v.clone(), v.clone())
///     }
/// }
/// */
///
/// fn bar() {
///     let foo = Foo::splat(-5);
///     assert_eq!(foo.0, -5);
///     assert_eq!(foo.1, -5);
/// }
/// ```

#[proc_macro_derive(Splat)]
pub fn derive_splat(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    // ensure that we are deriving a struct
    let data_struct = match input.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("Splat can only be derived by structs"),
    };

    // get the name of the struct we are deriving
    let struct_name = input.ident;

    proc_macro::TokenStream::from(match data_struct.fields {
        Fields::Named(fields_named) => {
            let shared_type = get_shared_type(fields_named.named.iter());
            let field_idents = fields_named.named.into_iter().map(|field| field.ident);

            quote!(
                impl #struct_name {
                    fn splat(v: #shared_type) -> Self {
                        Self {
                            #(#field_idents: v.clone()),*
                        }
                    }
                }
            )
        }
        Fields::Unnamed(fields_unnamed) => {
            let shared_type = get_shared_type(fields_unnamed.unnamed.iter());
            let field_idents = fields_unnamed.unnamed.into_iter().map(|field| field.ident);

            quote!(
                impl #struct_name {
                    fn splat(v: #shared_type) -> Self {
                        // we don't actually need the field_idents here, we just need the repetition of the iterator
                        Self(#(#field_idents v.clone()),*)
                    }
                }
            )
        }
        Fields::Unit => panic!("Splat cannot be derived by unit structs"),
    })
}

// gets the type of all of the fields in the struct
fn get_shared_type(mut fields: Iter<Field>) -> Type {
    // get the type of the first field
    let shared_type = match fields.next() {
        Some(first_field) => first_field.ty.clone(),
        None => panic!("Splat cannot be derived by structs with no fields"),
    };

    // ensure each other field is also of this type
    fields.for_each(|field| {
        if field.ty != shared_type {
            panic!("Splat can only be derived by structs where each field is the same type");
        }
    });

    shared_type
}
