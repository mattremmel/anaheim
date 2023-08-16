use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    parse_str, punctuated::Punctuated, token::Comma, FnArg, ImplItem, Item, ItemImpl, ItemStruct,
    ItemTrait, Pat, PatIdent, Result, Signature, TraitItem, Type, Visibility,
};

use shared::DelegateTraitMethodImpl;

use crate::{
    struct_new::{expand_delegate_new_impl, expand_struct_new_impl},
    syn_utils::parse_attr_args,
};

pub fn expand_service_attribute(attr_args: TokenStream, item: Item) -> Result<TokenStream> {
    match item {
        Item::Struct(input) => expand_service_struct(attr_args, input),
        Item::Impl(input) => expand_service_impl(attr_args, input),
        Item::Trait(input) => expand_service_trait(attr_args, input),
        _ => panic!("Service attribute not supported on this type"),
    }
}

fn expand_service_struct(
    attr_args: TokenStream,
    mut item_struct: ItemStruct,
) -> Result<TokenStream> {
    let _attr_args = parse_attr_args(attr_args)?;

    let service_vis = item_struct.vis.clone();
    let service_name = item_struct.ident.clone();
    let service_impl_name = format_ident!("{}Impl", &service_name);
    let service_trait_name = format_ident!("{}Trait", &service_name);
    let service_mock_name = format_ident!("Mock{}", &service_trait_name);

    item_struct.ident = service_impl_name.clone();

    let service_new_impl = expand_delegate_new_impl(&item_struct, &service_name);
    let item_struct_new_impl = expand_struct_new_impl(&item_struct);

    Ok(quote! {
        #[derive(Clone)]
        #service_vis struct #service_name {
            inner: ::std::sync::Arc<dyn #service_trait_name + Send + Sync>,
        }

        #service_new_impl

        impl #service_name {
            // TODO: Does this need to handle generics and docs and lints also?
            #[cfg(test)]
            fn from_mock(mock: #service_mock_name) -> Self {
                Self {
                    inner: ::std::sync::Arc::new(mock),
                }
            }
        }

        // TODO: Does this need to handle generics and docs and lints also?
        // TODO: I don't think generics is going to work easy here, because we would need additional parameters
        // TODO: On impl, and also need to know which ones would be on the right, and if there are constraints
        // TODO: But who knows, maybe it would be the same as parse, just plus the additional T for From
        impl<T> ::std::convert::From<T> for #service_name
        where T: #service_trait_name + Send + Sync
        {
            fn from(value: T) -> Self {
                Self {
                    inner: ::std::sync::Arc::new(value),
                }
            }
        }

        #item_struct

        #item_struct_new_impl
    })
}

fn expand_service_impl(attr_args: TokenStream, mut item_impl: ItemImpl) -> Result<TokenStream> {
    let _attr_args = parse_attr_args(attr_args)?;

    let service_name = match &*item_impl.self_ty {
        // TODO: I don't like this
        Type::Path(tp) => tp.path.get_ident().unwrap().clone(),
        _ => panic!("Service attribute doesn't support type"),
    };
    let service_impl_name = format_ident!("{}Impl", &service_name);
    let service_trait_name = format_ident!("{}Trait", &service_name);

    // TODO: I think we can replace this with quote_parse or something
    item_impl.self_ty = Box::new(parse_str::<Type>(&service_impl_name.to_string())?);

    // TODO: Use iter instead of vec and loop
    // TODO: Option to opt-out public method from trait
    // TODO: Option to set pub func without receiver as default trait impl
    let mut trait_methods = Vec::new();
    let mut non_trait_methods = Vec::new();
    for item in item_impl.items {
        match item {
            ImplItem::Fn(mut method) => {
                if matches!(method.vis, Visibility::Public(_)) {
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

    let delegate_trait_method_sigs = trait_methods.iter().map(|f| f.sig.clone());

    let delegate_trait_method_impls = trait_methods
        .iter()
        .map(|f| DelegateTraitMethodImpl(f.sig.clone()));

    Ok(quote! {
        // TODO: Option for private trait
        // TODO: Option for sealed trait?
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

fn expand_service_trait(attr_args: TokenStream, mut item_trait: ItemTrait) -> Result<TokenStream> {
    let _attr_args = parse_attr_args(attr_args)?;

    let service_vis = item_trait.vis.clone();
    let service_name = item_trait.ident;
    let service_trait_name = format_ident!("{}Trait", &service_name);
    let service_trait_items = &item_trait.items;
    let delegate_trait_method_impls = item_trait.items.iter().filter_map(|i| {
        if let TraitItem::Fn(f) = i {
            Some(DelegateTraitMethodImpl(f.sig.clone()))
        } else {
            None
        }
    });

    let service_mock_name = format_ident!("Mock{}", &service_trait_name);
    // TODO: Generics and associated types?

    item_trait.ident = service_trait_name.clone();

    Ok(quote! {
        #[derive(Clone)]
        #service_vis struct #service_name {
            inner: ::std::sync::Arc<dyn #service_trait_name + Send + Sync>,
        }

        impl #service_name {
            // TODO: Does this need to handle generics and docs and lints also?
            #[cfg(test)]
            fn from_mock(mock: #service_mock_name) -> Self {
                Self {
                    inner: ::std::sync::Arc::new(mock),
                }
            }
        }

        #[cfg(test, ::anaheim::automock)]
        #[::anaheim::async_trait]
        #service_vis trait #service_trait_name {
            #(#service_trait_items)*
        }

        #[::anaheim::async_trait]
        impl #service_trait_name for #service_name {
            #(#delegate_trait_method_impls)*
        }

        // TODO: Does this need to handle generics and docs and lints also?
        // TODO: I don't think generics is going to work easy here, because we would need additional parameters
        // TODO: On impl, and also need to know which ones would be on the right, and if there are constraints
        // TODO: But who knows, maybe it would be the same as parse, just plus the additional T for From
        impl<T> ::std::convert::From<T> for #service_name
        where T: #service_trait_name + Send + Sync
        {
            fn from(value: T) -> Self {
                Self {
                    inner: ::std::sync::Arc::new(value),
                }
            }
        }
    })
}

mod shared {
    use super::*;

    pub struct DelegateTraitMethodImpl(pub Signature);

    impl ToTokens for DelegateTraitMethodImpl {
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
}
