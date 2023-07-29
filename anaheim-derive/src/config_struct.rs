use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;
use syn::Result;

pub fn expand_config_struct(attr_args: TokenStream, item: Item) -> Result<TokenStream> {
    Ok(quote!(
        struct Config {}
    ))
}
