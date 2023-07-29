use crate::service_impl::ServiceTraitMethodImpl;
use crate::syn_utils::parse_attr_args;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemTrait, Result, TraitItem};

pub fn expand_service_trait(
    attr_args: TokenStream,
    mut item_trait: ItemTrait,
) -> Result<TokenStream> {
    let _attr_args = parse_attr_args(attr_args)?;

    let service_vis = item_trait.vis.clone();
    let service_name = item_trait.ident;
    let service_trait_name = format_ident!("{}Trait", &service_name);
    let service_trait_items = &item_trait.items;
    let delegate_trait_method_impls = item_trait.items.iter().filter_map(|i| {
        if let TraitItem::Fn(f) = i {
            Some(ServiceTraitMethodImpl(f.sig.clone()))
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

        #[cfg(test, ::anaheim::automock)]
        #[::anaheim::async_trait]
        #service_vis trait #service_trait_name {
            #(#service_trait_items)*
        }
    })
}
