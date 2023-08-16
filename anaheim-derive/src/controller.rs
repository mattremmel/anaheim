use proc_macro2::TokenStream;
use quote::quote;
use syn::{ImplItem, Item, ItemImpl, ItemStruct, Result};

use crate::syn_utils::parse_attr_args;

pub fn expand_controller_attribute(attr_args: TokenStream, item: Item) -> Result<TokenStream> {
    match item {
        Item::Struct(input) => expand_controller_struct(attr_args, input),
        Item::Impl(input) => expand_controller_impl(attr_args, input),
        _ => panic!("Controller attribute not supported on this type"),
    }
}

fn expand_controller_struct(
    attr_args: TokenStream,
    mut item_struct: ItemStruct,
) -> Result<TokenStream> {
    let _attr_args = parse_attr_args(attr_args)?;
    Ok(quote!(#item_struct))
}

fn expand_controller_impl(attr_args: TokenStream, mut item_impl: ItemImpl) -> Result<TokenStream> {
    let _attr_args = parse_attr_args(attr_args)?;

    for item in &mut item_impl.items {
        if let ImplItem::Fn(method) = item {
            for attr in &mut method.attrs {
                if attr.meta.
            }
        }
    }

    Ok(quote! {
        #[::anaheim::web::OpenApi]
        #item_impl
    })
}
