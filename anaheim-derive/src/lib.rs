use crate::config_struct::expand_config_struct;
use crate::service_impl::expand_service_impl;
use crate::service_struct::expand_service_struct;
use crate::service_trait::expand_service_trait;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};
use syn_utils::into_macro_output;

mod config_struct;
mod service_impl;
mod service_shared;
mod service_struct;
mod service_trait;
mod struct_new;
mod syn_utils;

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
pub fn service(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    match parse_macro_input!(item as Item) {
        Item::Struct(input) => into_macro_output(expand_service_struct(attr_args.into(), input)),
        Item::Impl(input) => into_macro_output(expand_service_impl(attr_args.into(), input)),
        Item::Trait(input) => into_macro_output(expand_service_trait(attr_args.into(), input)),
        _ => panic!("Service attribute not supported on this type"),
    }
}
