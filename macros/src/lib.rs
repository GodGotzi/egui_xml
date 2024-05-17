extern crate proc_macro;

use std::{cell::RefCell, rc::Rc};

use layout::strip::expand_strip;
use parser::{Form, Node};
use proc_macro::TokenStream;

use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, LitStr};

mod layout;

fn expand_nodes(children: &[Rc<RefCell<Node>>]) -> Result<proc_macro2::TokenStream, String> {
    let mut expanded = quote! {};

    for node in children.iter() {
        let node_expanded = expand_node(node)?;
        expanded.append_all(node_expanded);
    }

    Ok(expanded)
}

fn expand_node(node: &Rc<RefCell<Node>>) -> Result<proc_macro2::TokenStream, String> {
    match &*node.borrow() {
        parser::Node::Panel { children, .. } => expand_nodes(children),
        parser::Node::Rust { code, .. } => Ok(code.parse().unwrap()),
        parser::Node::Border { .. } => Ok(quote! {}),
        parser::Node::Grid { .. } => Ok(quote! {}),
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

    let form: Form = match xml.try_into() {
        Ok(form) => form,
        Err(_) => panic!("Failed to load XML"),
    };

    let expanded = match expand_nodes(&form.nodes) {
        Ok(expanded) => expanded,
        Err(e) => panic!("{}", e),
    };

    expanded.into()
}

#[proc_macro]
pub fn load_layout_file(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as LitStr);
    let file_path = input.value();

    let file_content =
        std::fs::read_to_string(&file_path).expect(&format!("unable to find {}", file_path));

    load_layout(file_content.parse().unwrap())
}
