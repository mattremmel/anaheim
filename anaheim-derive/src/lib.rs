use proc_macro::TokenStream;

use syn::{parse_macro_input, Item};

use syn_utils::into_macro_output;

use crate::config::expand_config_struct;
use crate::controller::expand_controller_attribute;
use crate::service::expand_service_attribute;

mod config;
mod controller;
mod service;
mod struct_new;
mod syn_utils;

// TODO: Use darling instead of struct_meta

// TODO: Can we use quote_spanned anywhere for better error messages?
// TODO: Add #[doc = ...] attributes
// TODO: Add lint_attrs if necessary

#[proc_macro_attribute]
pub fn config(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    into_macro_output(expand_config_struct(
        attr_args.into(),
        parse_macro_input!(item as Item),
    ))
}

#[proc_macro_attribute]
pub fn controller(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    into_macro_output(expand_controller_attribute(
        attr_args.into(),
        parse_macro_input!(item as Item),
    ))
}

#[proc_macro_attribute]
pub fn service(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    into_macro_output(expand_service_attribute(
        attr_args.into(),
        parse_macro_input!(item as Item),
    ))
}
