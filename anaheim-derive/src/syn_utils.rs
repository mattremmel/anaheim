use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use structmeta::{Parse, ToTokens};
use syn::{parse2, punctuated::Punctuated, token::Comma, Expr, Result, Token};

// TODO: Make this a trait so then it can be an extension
pub fn into_macro_output(input: Result<TokenStream2>) -> TokenStream1 {
    match input {
        Ok(s) => s,
        Err(e) => e.into_compile_error(),
    }
    .into()
}

pub fn parse_attr_args(input: TokenStream2) -> Result<Option<Args>> {
    if !input.is_empty() {
        parse2::<Args>(input).map(|a| Ok(Some(a)))?
    } else {
        Ok(None)
    }
}

#[derive(Parse)]
pub struct Args(#[parse(terminated)] pub Punctuated<Arg, Comma>);

#[derive(ToTokens, Parse)]
pub enum Arg {
    NameValue {
        #[parse(peek, any)]
        name: Ident,
        #[parse(peek)]
        eq_token: Token!(=),
        value: Expr,
    },
    Value(Expr),
}
