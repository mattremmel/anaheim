use proc_macro2::TokenStream;
use syn::{ItemImpl, Result};

use crate::syn_utils::parse_attr_args;

pub fn expand_controller_impl(
    attr_args: TokenStream,
    mut item_impl: ItemImpl,
) -> Result<TokenStream> {
    let _attr_args = parse_attr_args(attr_args)?;
    todo!()
}
