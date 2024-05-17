extern crate proc_macro;

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use layout::strip::expand_strip;
use parser::{Form, Node};
use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::{quote, TokenStreamExt};

mod layout;
mod utils;

fn expand_nodes(children: &[Rc<RefCell<Node>>]) -> proc_macro2::TokenStream {
    let mut expanded = quote! {};

    for node in children.iter() {
        let node_expanded = expand_node(node);
        expanded.append_all(node_expanded);
    }

    expanded
}

fn expand_node(node: &Rc<RefCell<Node>>) -> proc_macro2::TokenStream {
    match &*node.borrow() {
        parser::Node::Panel { children, .. } => expand_nodes(children),
        parser::Node::Rust {
            attributes, code, ..
        } => code.parse().unwrap(),
        parser::Node::Border {
            children,
            attributes,
            ..
        } => quote! {},
        parser::Node::Grid {
            children,
            attributes,
            ..
        } => quote! {},
        parser::Node::Default { children, .. } => expand_nodes(children),
        parser::Node::Strip {
            children,
            attributes,
            ..
        } => expand_strip(children, attributes),
    }
}

#[proc_macro]
pub fn load_layout(input: TokenStream) -> TokenStream {
    let xml = input.to_string();

    let form: Form = xml.try_into().unwrap();

    let expanded = expand_nodes(&form.nodes);

    expanded.into()
}
