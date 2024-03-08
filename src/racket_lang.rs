extern crate proc_macro;
use proc_macro::*;

mod special_fn;

fn internal_fn(fn_name: &String, params: &Vec<TokenTree>) -> Option<String> {
    match fn_name.as_str() {
        "+" => special_fn::add(params),
        "-" => special_fn::sub(params),
        "*" => special_fn::mul(params),
        "/" => special_fn::div(params),
        "define" => special_fn::define(params),
        "lambda" => special_fn::lambda(params),
        _ => None,
    }
}

fn tokentree_group(group: &Group) -> String {
    let mut it = group.stream().into_iter();
    let function = it.next().unwrap();
    let fn_name = tokentree(&function);
    let parameters = it.collect();

    if let Some(called) = internal_fn(&fn_name, &parameters) {
        return called;
    }

    let mut param_str: String = parameters
        .iter()
        .map(|tt| tokentree(tt))
        .map(|s| format!("{}, ", s))
        .flat_map(|s| s.into_bytes())
        .map(|b| b as char)
        .collect();
    param_str.pop();
    param_str.pop();

    format!("{}({})", fn_name, param_str)
}

fn tokentree_ident(ident: &Ident) -> String {
    ident.to_string()
}

fn tokentree_punct(punct: &Punct) -> String {
    punct.to_string()
}

fn tokentree_literal(literal: &Literal) -> String {
    literal.to_string()
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
