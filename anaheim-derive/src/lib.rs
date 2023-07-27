use crate::config::expand_config_token_stream;
use crate::service::expand_service_token_stream;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};
use syn_utils::into_macro_output;

mod config;
mod service;
mod syn_utils;

// TODO: Can we use quote_spanned anywhere for better error messages?

#[proc_macro_attribute]
pub fn config(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    into_macro_output(expand_config_token_stream(
        attr_args.into(),
        parse_macro_input!(item as Item),
    ))
}

#[proc_macro_attribute]
pub fn service(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    into_macro_output(expand_service_token_stream(
        attr_args.into(),
        parse_macro_input!(item as Item),
    ))
}
