extern crate proc_macro;
use proc_macro::*;

fn tokentree_group(group: &Group) -> String {
    let s = format!("[Group:{}]\n", group.to_string());
    format!("{}{}", s, tokenstream(&group.stream()))
}

fn tokentree_ident(ident: &Ident) -> String {
    format!("[Ident:{}]", ident.to_string())
}

fn tokentree_punct(punct: &Punct) -> String {
    format!("[Punct:{}]", punct.to_string())
}

fn tokentree_literal(literal: &Literal) -> String {
    format!("[Literal:{}]", literal.to_string())
}

fn tokentree(tt: &TokenTree) -> String {
    match tt {
        TokenTree::Group(group) => tokentree_group(group),
        TokenTree::Ident(ident) => tokentree_ident(ident),
        TokenTree::Punct(punct) => tokentree_punct(punct),
        TokenTree::Literal(literal) => tokentree_literal(literal),
    }
}

pub(super) fn tokenstream(ts: &TokenStream) -> String {
    let mut s = String::from("");
    for tree in ts.clone().into_iter() {
        s = format!("{}{}\n", s, tokentree(&tree));
    }
    s
}
