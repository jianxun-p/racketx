extern crate proc_macro;
use proc_macro::*;

mod dbgstring;

mod racket_lang;

#[proc_macro]
pub fn print_tokenstream(ts: TokenStream) -> TokenStream {
    format!("println!(\"{}\")", ts).parse().unwrap()
}

#[proc_macro]
pub fn dbgstringify(ts: TokenStream) -> TokenStream {
    format!("\"{}\"", dbgstring::tokenstream(&ts))
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn racket_println(ts: TokenStream) -> TokenStream {
    format!("println!(\"{}\")", racket_lang::tokenstream(&ts))
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn racket(ts: TokenStream) -> TokenStream {
    racket_lang::tokenstream(&ts).parse().unwrap()
}
