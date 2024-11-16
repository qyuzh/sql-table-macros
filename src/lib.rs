use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser, parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Field, Fields,
    Token, Type,
};

/// Auto add `created_by(owner_type)`, `updated_by(owner_type)`, `created_at(time_type)`, `updated_at(time_type)`
/// and `is_deleted(bool)` fields to struct
///
/// The format is
/// - `crud(owner_and_time_type)`
/// - `crud(owner_type, time_type)`
///
/// owner default type id `u64`, time default type is `String`
///
#[proc_macro_attribute]
pub fn table_common_fields(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_parsed = parse_macro_input!(args as CommaSeparatedTypes);
    let owner_type = args_parsed
        .types
        .first()
        .cloned()
        .unwrap_or(Type::Verbatim(quote! { u64 }));
    let time_type = args_parsed
        .types
        .last()
        .cloned()
        .unwrap_or(Type::Verbatim(quote! { String }));

    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let expanded = match input.data {
        Data::Struct(mut data_struct) => {
            if let Fields::Named(ref mut named_fields) = data_struct.fields {
                let fields_to_add = vec![
                    ("created_by", quote! { pub created_by: #owner_type }),
                    ("updated_by", quote! { pub updated_by: #owner_type }),
                    ("created_at", quote! { pub created_at: #time_type }),
                    ("updated_at", quote! { pub updated_at: #time_type }),
                    ("is_deleted", quote! { pub is_deleted: bool }),
                ];

                for (field_name, field_quote) in fields_to_add {
                    let already_has_field = named_fields.named.iter().any(|field| {
                        if let Some(ident) = &field.ident {
                            ident == field_name
                        } else {
                            false
                        }
                    });

                    if !already_has_field {
                        named_fields
                            .named
                            .push(Field::parse_named.parse2(field_quote).unwrap());
                    }
                }
            }

            let fields = &data_struct.fields;
            let attrs = &input.attrs;
            quote! {
                #(#attrs)*
                pub struct #struct_name #fields
            }
        }
        _ => syn::Error::new_spanned(input, "AddField can only be used on structs.")
            .to_compile_error(),
    };

    TokenStream::from(expanded)
}

struct CommaSeparatedTypes {
    types: Punctuated<Type, Token![,]>,
}

// Implement parsing for the helper struct
impl syn::parse::Parse for CommaSeparatedTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(CommaSeparatedTypes {
            types: Punctuated::parse_terminated(input)?,
        })
    }
}
