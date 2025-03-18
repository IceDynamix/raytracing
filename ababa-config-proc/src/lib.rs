use proc_macro::TokenStream;

use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(AbabaDeserialize)]
pub fn derive_deser_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let Data::Struct(struc) = input.data else {
        return syn::Error::new(input.span(), "Can only implement resource on struct")
            .to_compile_error()
            .into();
    };

    let Fields::Named(fields) = struc.fields else {
        return syn::Error::new(
            struc.fields.span(),
            "Can only implement resource on struct with named fields",
        )
        .to_compile_error()
        .into();
    };

    let each_field: Vec<_> = fields
        .named
        .into_iter()
        .filter_map(|f| f.ident)
        .map(|f| {
            quote! {
                #f: fields.remove(stringify!(#f))
                    .ok_or(ababa_config::AbabaParseError::StructFieldNotPresent { field: stringify!(#f) })?
                    .try_into()?
            }
        })
        .collect();

    let expanded = quote! {
        impl TryFrom<ababa_config::AbabaValue> for #struct_name {
            type Error = ababa_config::AbabaParseError;

            fn try_from(value: ababa_config::AbabaValue) -> Result<Self, Self::Error> {
                match value {
                    ababa_config::AbabaValue::Object {
                        struct_type,
                        mut fields,
                    } => {
                        if struct_type.as_ref().is_some_and(|t| t != stringify!(#struct_name)) {
                            return Err(ababa_config::AbabaParseError::StructTypeDidNotMatch {
                                expected: stringify!(#struct_name),
                                got: struct_type,
                            });
                        }

                        Ok(Self {
                            #(#each_field), *
                        })
                    }
                    _ => Err(ababa_config::AbabaParseError::ValueTypeDidNotMatch {
                        expected: "Object",
                        got: value,
                    }),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
