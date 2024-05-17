extern crate proc_macro;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
        parser::Node::UiExecutable { attributes, .. } => expand_ui_executable(attributes),
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

fn expand_ui_executable(attributes: &HashMap<String, String>) -> proc_macro2::TokenStream {
    let ui_fn = match attributes.get("ident") {
        Some(ident) => proc_macro2::Ident::new(&ident, Span::call_site()),
        None => panic!(""),
    };

    quote! {
        #ui_fn (ui);
    }
}

#[proc_macro]
pub fn load_layout(input: TokenStream) -> TokenStream {
    let xml = input.to_string();

    let form: Form = xml.try_into().unwrap();

    let expanded = expand_nodes(&form.nodes);

    expanded.into()
}
