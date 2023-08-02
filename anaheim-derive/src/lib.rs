use proc_macro::TokenStream;

use syn::{parse_macro_input, Item};

use syn_utils::into_macro_output;

use crate::config_struct::expand_config_struct;
use crate::controller_impl::expand_controller_impl;
use crate::controller_struct::expand_controller_struct;
use crate::service::expand_service_attribute;

mod config_struct;
mod controller_impl;
mod controller_struct;
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
    match parse_macro_input!(item as Item) {
        Item::Struct(input) => into_macro_output(expand_controller_struct(attr_args.into(), input)),
        Item::Impl(input) => into_macro_output(expand_controller_impl(attr_args.into(), input)),
        _ => panic!("Controller attribute not supported on this type"),
    }
}

#[proc_macro_attribute]
pub fn service(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    into_macro_output(expand_service_attribute(
        attr_args.into(),
        parse_macro_input!(item as Item),
    ))
}
