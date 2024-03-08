
extern crate proc_macro;
use proc_macro::*;



pub(crate) fn tokentree(tt: &TokenTree) -> String {
    match tt {
        TokenTree::Group(group) => group.to_string(),
        TokenTree::Ident(ident) => ident.to_string(),
        TokenTree::Punct(punct) => punct.to_string(),
        TokenTree::Literal(literal) => literal.to_string(),
    }
}

pub(super) fn tokenstream(ts: &TokenStream) -> String {
    let mut s = String::from("");
    for tree in ts.clone().into_iter() {
        s = format!("{}{} ", s, tokentree(&tree));
    }
    s
}

