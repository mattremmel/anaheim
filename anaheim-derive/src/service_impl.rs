use crate::syn_utils::parse_attr_args;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_str, FnArg, ImplItem, ItemImpl, Pat, PatIdent, Result, Signature, Type, Visibility,
};

pub fn expand_service_impl(attr_args: TokenStream, mut item_impl: ItemImpl) -> Result<TokenStream> {
    let _attr_args = parse_attr_args(attr_args)?;

    let service_name = match &*item_impl.self_ty {
        // TODO: I don't like this
        Type::Path(tp) => tp.path.get_ident().unwrap().clone(),
        _ => panic!("service doesn't support type"),
    };
    let service_impl_name = format_ident!("{}Impl", &service_name);
    let service_trait_name = format_ident!("{}Trait", &service_name);

    // TODO: I think we can replace this with quote_parse or something
    item_impl.self_ty = Box::new(parse_str::<Type>(&service_impl_name.to_string())?);

    // TODO: Use iter instead of vec and loop
    // TODO: If don't have self, then can maybe take whole body as a default?
    let mut trait_methods = Vec::new();
    let mut non_trait_methods = Vec::new();
    for item in item_impl.items {
        match item {
            ImplItem::Fn(mut method) => {
                if matches!(method.vis, Visibility::Public(_)) && method.sig.receiver().is_some() {
                    method.vis = Visibility::Inherited;
                    trait_methods.push(method);
                } else {
                    non_trait_methods.push(method)
                }
            }
            // TODO: Include span of the item
            _ => panic!("Service attribute not supported for non-function impl items"),
        }
    }

    let delegate_trait_method_sigs = trait_methods
        .iter()
        .map(|f| f.sig.clone());

    let delegate_trait_method_impls = trait_methods
        .iter()
        .map(|f| ServiceTraitMethodImpl(f.sig.clone()));

    Ok(quote! {
        // TODO: We need to figure out visibility. We don't have it on impl.
        // TODO: Need to be able to opt-out of automock
        #[cfg(test, ::anaheim::automock)]
        #[::anaheim::async_trait]
        pub trait #service_trait_name {
            #(#delegate_trait_method_sigs;)*
        }

        #[::anaheim::async_trait]
        impl #service_trait_name for #service_name {
            #(#delegate_trait_method_impls)*
        }

        #[::anaheim::async_trait]
        impl #service_trait_name for #service_impl_name {
            #(#trait_methods)*
        }

        impl #service_impl_name {
            #(#non_trait_methods)*
        }
    })
}

pub struct ServiceTraitMethodImpl(pub Signature);

impl ToTokens for ServiceTraitMethodImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);

        let method_name = self.0.ident.clone();
        let method_args = self
            .0
            .inputs
            .clone()
            .into_iter()
            .filter_map(|a| {
                if let FnArg::Typed(p) = a {
                    match &*p.pat {
                        Pat::Ident(ident) => Some(ident.clone()),
                        // TODO: Can we include span of pattern?
                        _ => panic!("Service attribute not supported for functions with non identifier argument patterns"),
                    }
                } else {
                    // ignore the receiver
                    None
                }
            })
            .collect::<Punctuated<PatIdent, Comma>>();

        let call_await = self.0.asyncness.map(|_| quote!(.await));

        tokens.append_all(quote! {
            {
                self.inner.#method_name(#method_args)#call_await
            }
        });
    }
}
