mod attr;

use crate::attr::{EnvAttr, EnvAttrOp};
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn serde_with_env(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut derive_input: DeriveInput = parse_macro_input!(item);

    let struct_name = &derive_input.ident;

    let mod_name = format_ident!("__serde_with_env__{}", struct_name);
    let shadow_struct_name = format_ident!("__{}", struct_name);
    let try_from_path = format!("{mod_name}::{shadow_struct_name}");

    derive_input
        .attrs
        .push(parse_quote!( #[serde(try_from = #try_from_path)]));

    quote_spanned! { derive_input.span() =>
        #[derive(serde_with_env::SerdeWithEnv)]
        #derive_input
    }
    .into_token_stream()
    .into()
}

// #[with_env(or = "")] - пробуем из env если из основного источника не получилось
// #[with_env(over = "")] - сначала env - потом из основного источника
// #[with_env(only = "")] - только из env

#[proc_macro_derive(SerdeWithEnv, attributes(with_env))]
pub fn serde_with_env_derive(item: TokenStream) -> TokenStream {
    let mut derive_input: DeriveInput = parse_macro_input!(item);
    let struct_name = &derive_input.ident;

    let mod_name = format_ident!("__serde_with_env__{}", struct_name);
    let shadow_struct_name = format_ident!("__{}", struct_name);

    let mut generated_get_env = Vec::new();
    let mut generated_from_fields = Vec::new();
    let mut generated_fields = Vec::new();

    if let Data::Struct(ref mut data) = derive_input.data {
        if let Fields::Named(ref mut fields) = data.fields {
            for field in fields.named.iter_mut() {
                let mut field_is_option = false;
                let field_name = field.ident.as_ref().unwrap();
                let maybe_attr = match attr::get_env_attr(&field.attrs) {
                    None => {
                        generated_fields.push(field.to_token_stream());
                        generated_from_fields.push(quote! {
                            #field_name: v.#field_name
                        });
                        continue;
                    }
                    Some(maybe_attr) => {
                        field.attrs.retain(|attr| !attr.path().is_ident("with_env"));

                        let mut field = field.clone();
                        let ty = field.ty.clone();

                        if let Type::Path(path) = &ty
                            && let Some(segment) = path.path.segments.first()
                            && segment.ident == "Option"
                        {
                            field_is_option = true;
                        }

                        if !field_is_option {
                            field.ty = parse_quote!(Option<#ty>);
                        }

                        if let Ok(EnvAttr {
                            op: EnvAttrOp::Or | EnvAttrOp::Over,
                            ..
                        }) = &maybe_attr
                        {
                            generated_fields.push(field.to_token_stream());
                        }
                        generated_from_fields.push(quote! {
                            #field_name
                        });
                        maybe_attr
                    }
                };

                match maybe_attr {
                    Ok(env_attr) => {
                        let env_name = env_attr.name.value();
                        let env_default = env_attr.default.as_ref();
                        let env_convert = env_attr.convert.as_ref();

                        let missing_err = format!("Missing \"{env_name}\" environment variable.");
                        let parse_err =
                            format!("Cannot parse \"{env_name}\" environment variable: {{err}}");

                        let field_as_some = match field_is_option {
                            true => quote! { Ok(Some(v)) },
                            false => quote! { Ok(v) },
                        };

                        let parse_as_some = match (field_is_option, env_convert) {
                            (true, None) => {
                                quote! { v.parse().map(Some).map_err(|err| format!(#parse_err)) }
                            }
                            (false, None) => {
                                quote! { v.parse().map_err(|err| format!(#parse_err)) }
                            }
                            (_, Some(convert)) => {
                                let ident = format_ident!("{}", convert.value());
                                quote! { #ident(v) }
                            }
                        };

                        let not_present_error = match (field_is_option, env_default) {
                            (true, Some(default)) => quote! { Ok(Some(#default.into())) },
                            (false, Some(default)) => quote! { Ok(#default.into()) },
                            (true, None) => quote! { Ok(None)},
                            (false, None) => quote! { Err(#missing_err.to_string()) },
                        };

                        let env_strategy = match env_attr.op {
                            EnvAttrOp::Or => {
                                quote! {
                                    let #field_name = if let Some(v) = v.#field_name {
                                        #field_as_some
                                    } else {
                                        match std::env::var(#env_name) {
                                            Ok(v) => #parse_as_some,
                                            Err(err) => #not_present_error,
                                        }
                                    }?;
                                }
                            }
                            EnvAttrOp::Only => {
                                quote! {
                                    let #field_name  = match std::env::var(#env_name) {
                                        Ok(v) => #parse_as_some,
                                        Err(err) => #not_present_error,
                                    }?;
                                }
                            }
                            EnvAttrOp::Over => {
                                let not_present_error = match (field_is_option, env_default) {
                                    (false, None) => quote! { Err(#missing_err.to_string()) },
                                    _ => not_present_error,
                                };

                                quote! {
                                    let #field_name = match std::env::var(#env_name) {
                                        Ok(v) => #parse_as_some,
                                        Err(_) => match v.#field_name {
                                            None => #not_present_error,
                                            Some(v) => #field_as_some,
                                        },
                                    }?;
                                }
                            }
                        };

                        generated_get_env.push(env_strategy);
                    }
                    Err(err) => {
                        let err = err.into_compile_error();
                        generated_get_env.push(quote! {
                            compile_error!(#err);
                        });
                    }
                }
            }
        }
    }

    let try_from_impl = quote! {
        #[automatically_derived]
        impl TryFrom<#shadow_struct_name> for #struct_name {
            type Error = String;
            fn try_from(v: #shadow_struct_name) -> Result<Self, Self::Error> {
                #(#generated_get_env)*

                Ok(Self {
                    #(#generated_from_fields),*
                })
            }
        }
    };

    quote! {
        #[allow(non_snake_case)]
        mod #mod_name {
            use super::*;

            #[derive(serde::Deserialize)]
            pub struct #shadow_struct_name {
                #(#generated_fields),*
            }

            #try_from_impl
        }
    }
    .into()
}
