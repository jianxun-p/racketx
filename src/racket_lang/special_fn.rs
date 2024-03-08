extern crate proc_macro;
use proc_macro::*;

use super::tokentree;

pub(super) fn add(params: &Vec<TokenTree>) -> Option<String> {
    let mut ans: String = params
        .iter()
        .map(|tt| tokentree(tt))
        .map(|s| format!("{}+", s))
        .flat_map(|s| s.into_bytes())
        .map(|b| b as char)
        .collect();
    ans.push('0');
    Some(ans)
}

pub(super) fn sub(params: &Vec<TokenTree>) -> Option<String> {
    let ans: String = params
        .iter()
        .map(|tt| tokentree(tt))
        .map(|s| format!("-{}", s))
        .flat_map(|s| s.into_bytes())
        .map(|b| b as char)
        .collect();
    Some(format!("0{}", ans))
}

pub(super) fn mul(params: &Vec<TokenTree>) -> Option<String> {
    let mut ans: String = params
        .iter()
        .map(|tt| tokentree(tt))
        .map(|s| format!("{}*", s))
        .flat_map(|s| s.into_bytes())
        .map(|b| b as char)
        .collect();
    ans.push('1');
    Some(ans)
}

pub(super) fn div(params: &Vec<TokenTree>) -> Option<String> {
    let mut ans: String = params
        .iter()
        .map(|tt| tokentree(tt))
        .map(|s| format!("{}/", s))
        .flat_map(|s| s.into_bytes())
        .map(|b| b as char)
        .collect();
    ans.push('1');
    Some(ans)
}

fn define_const(ident: &Ident, value: &TokenTree) -> Option<String> {
    Some(format!("let {} = {};\n", ident, tokentree(value)))
}

pub(super) fn define(params: &Vec<TokenTree>) -> Option<String> {
    if params.len() != 2 {
        None
    } else if let TokenTree::Ident(ident) = &params[0] {
        define_const(ident, &params[1])
    } else {
        None
    }
}

pub(super) fn lambda(params: &Vec<TokenTree>) -> Option<String> {
    if params.len() != 2 {
        None
    } else if let TokenTree::Group(group) = &params[0] {
        let mut lambda_param: String = group
            .stream()
            .into_iter()
            .map(|tt| format!("{}, ", tokentree(&tt)))
            .flat_map(|s| s.into_bytes())
            .map(|b| b as char)
            .collect();
        lambda_param.pop();
        lambda_param.pop();
        Some(format!("(|{}| {})", lambda_param, tokentree(&params[1])))
    } else {
        None
    }
}
