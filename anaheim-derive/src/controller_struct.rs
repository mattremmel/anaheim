use proc_macro2::TokenStream;
use syn::{ItemStruct, Result};

use crate::syn_utils::parse_attr_args;

pub fn expand_controller_struct(
    attr_args: TokenStream,
    mut item_struct: ItemStruct,
) -> Result<TokenStream> {
    let _attr_args = parse_attr_args(attr_args)?;
    todo!()
}
