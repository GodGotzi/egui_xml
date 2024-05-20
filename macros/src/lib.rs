//! `load_layout!` macro for loading layout from XML for egui
//!
//! The `load_layout!` macro allows for loading layout configurations from XML for use with the egui GUI framework. It takes an XML representation of the layout structure and generates Rust code to construct the layout within an egui UI.
//!
//! # Example
//!
//! ```rust
//! # use egui_macros::load_layout;
//! let layout_code = load_layout!(
//!     <Strip direction="west">
//!         <Panel size="relative" value="0.3">
//!             color_background(ui, egui::Color32::from_rgb(0, 0, 255));
//!         </Panel>
//!         <Panel size="remainder">
//!             <Strip direction="north">
//!                 <Panel size="relative" value="0.3">
//!                     color_background(ui, egui::Color32::from_rgb(0, 255, 255));
//!                 </Panel>
//!                 <Panel size="remainder">
//!                     color_background(ui, egui::Color32::from_rgb(255, 0, 255));
//!                 </Panel>
//!             </Strip>
//!         </Panel>
//!     </Strip>
//! );
//! ```

extern crate proc_macro;

use std::{cell::RefCell, rc::Rc};

use layout::strip::expand_strip;
use parser::{Node, XMLForm};
use proc_macro::TokenStream;

use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, LitStr};

mod layout;

struct XMLContext;

fn expand_nodes(
    children: &[Rc<RefCell<Node>>],
    ctx: &XMLContext,
) -> Result<proc_macro2::TokenStream, String> {
    let mut expanded = quote! {};

    for node in children.iter() {
        let node_expanded = expand_node(node, ctx)?;
        expanded.append_all(node_expanded);
    }

    Ok(expanded)
}

fn expand_node(
    node: &Rc<RefCell<Node>>,
    ctx: &XMLContext,
) -> Result<proc_macro2::TokenStream, String> {
    match &*node.borrow() {
        parser::Node::Panel { children, .. } => expand_nodes(children, ctx),
        parser::Node::Rust { code, .. } => Ok(code.parse().unwrap()),
        parser::Node::Border { .. } => Ok(quote! {}),
        parser::Node::Grid { .. } => Ok(quote! {}),
        parser::Node::Default { children, .. } => expand_nodes(children, ctx),
        parser::Node::Strip { .. } => expand_strip(node, ctx),
    }
}

/// Macro for loading layout from XML.
///
/// This macro parses an XML layout representation and generates Rust code to construct the layout within an egui UI.
///
/// # Example
///
/// ```rust
/// load_layout!(
///     <Strip direction="west">
///         <Panel size="relative" value="0.3">
///             color_background(ui, egui::Color32::from_rgb(0, 0, 255));
///         </Panel>
///         <Panel size="remainder">
///             <Strip direction="north">
///                 <Panel size="relative" value="0.3">
///                     color_background(ui, egui::Color32::from_rgb(0, 255, 255));
///                 </Panel>
///                 <Panel size="remainder">
///                     color_background(ui, egui::Color32::from_rgb(255, 0, 255));
///                 </Panel>
///             </Strip>
///         </Panel>
///     </Strip>
/// );
/// ```
#[proc_macro]
pub fn load_layout(input: TokenStream) -> TokenStream {
    let xml = input.to_string();

    let form: XMLForm = match xml.try_into() {
        Ok(form) => form,
        Err(_) => panic!("Failed to load XML"),
    };

    let ctx = XMLContext;

    let expanded = match expand_node(&form.root, &ctx) {
        Ok(expanded) => expanded,
        Err(e) => panic!("{}", e),
    };

    expanded.into()
}

/// Macro for loading layout from a file.
///
/// This macro reads the content of the specified file and passes it to the `load_layout` macro for parsing and code generation.
///
/// # Example
///
/// ```rust
/// load_layout_file!("layout.xml");
/// ```
#[proc_macro]
pub fn load_layout_file(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as LitStr);
    let file_path = input.value();

    let file_content =
        std::fs::read_to_string(&file_path).expect(&format!("unable to find {}", file_path));

    load_layout(file_content.parse().unwrap())
}
