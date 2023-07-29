use crate::struct_new::{expand_delegate_new_impl, expand_struct_new_impl};
use crate::syn_utils::parse_attr_args;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemStruct, Result};

pub fn expand_service_struct(
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
