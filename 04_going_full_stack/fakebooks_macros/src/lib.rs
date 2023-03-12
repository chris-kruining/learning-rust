#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, Token, Ident, LitStr, token::Paren};
use derive_syn_parse::Parse;

#[derive(Parse)]
enum Custom {
    // #[peek(LitStr, name = "")]
    // LitStr(LitStr),
    #[peek(Ident, name = "")]
    MacroCall(MacroCall),
}

#[derive(Parse)]
struct MacroCall {
    macro_ident: Ident,
    bang_token: Token!(!),
    #[paren]
    paren: Paren,
    #[inside(paren)]
    ident: Ident,
}

#[proc_macro]
#[proc_macro_error]
pub fn kaas(_tokens: TokenStream) -> TokenStream {
    

    
    "10".parse().unwrap()
}
