use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};

use parser::Node;
use proc_macro2::Span;
use quote::{quote, TokenStreamExt};

use quote_into::quote_into;

use strum_macros::EnumString;

#[derive(PartialEq, Eq, EnumString)]
enum StripDirection {
    #[strum(
        serialize = "BottomUp",
        serialize = "bottomup",
        serialize = "bu",
        serialize = "south",
        serialize = "s"
    )]
    BottomUp,
    #[strum(
        serialize = "LeftToRight",
        serialize = "lefttoright",
        serialize = "ltr",
        serialize = "west",
        serialize = "w"
    )]
    LeftToRight,
    #[strum(
        serialize = "RightToLeft",
        serialize = "righttoleft",
        serialize = "rtl",
        serialize = "east",
        serialize = "e"
    )]
    RightToLeft,
    #[strum(
        serialize = "TopDown",
        serialize = "topdown",
        serialize = "td",
        serialize = "north",
        serialize = "n"
    )]
    TopDown,
}

struct StripInfo {
    direction: StripDirection,
}

impl TryFrom<&HashMap<String, String>> for StripInfo {
    type Error = String;

    fn try_from(attributes: &HashMap<String, String>) -> Result<Self, Self::Error> {
        if let Some(dir) = attributes.get("direction") {
            if let Ok(strip_direction) = StripDirection::from_str(dir) {
                return Ok(StripInfo {
                    direction: strip_direction,
                });
            } else {
                return Err("direction type doesn't exist!".to_string());
            }
        }

        Err("direction attribute doesn't exist!".to_string())
    }
}

#[derive(EnumString)]
enum StripChildSize {
    #[strum(serialize = "Remainder", serialize = "remainder")]
    Remainder,
    #[strum(serialize = "Exact", serialize = "exact")]
    Exact(f32),
    #[strum(serialize = "Initial", serialize = "initial")]
    Initial(f32),
    #[strum(serialize = "Relative", serialize = "relative")]
    Relative(f32),
}

struct StripChildInfo {
    size: StripChildSize,
}

impl TryFrom<&HashMap<String, String>> for StripChildInfo {
    type Error = String;

    fn try_from(attributes: &HashMap<String, String>) -> Result<Self, Self::Error> {
        if let Some(size) = attributes.get("size") {
            if let Ok(child_size) = StripChildSize::from_str(size) {
                return match child_size {
                    StripChildSize::Remainder => {
                        return Ok(StripChildInfo {
                            size: StripChildSize::Remainder,
                        });
                    }
                    _ => {
                        if let Some(value) = attributes.get("value") {
                            if let Ok(value) = value.parse::<f32>() {
                                return match child_size {
                                    StripChildSize::Exact(_) => Ok(StripChildInfo {
                                        size: StripChildSize::Exact(value),
                                    }),
                                    StripChildSize::Initial(_) => Ok(StripChildInfo {
                                        size: StripChildSize::Initial(value),
                                    }),
                                    StripChildSize::Relative(_) => Ok(StripChildInfo {
                                        size: StripChildSize::Relative(value),
                                    }),
                                    _ => panic!("Why you here!"),
                                };
                            }
                        }

                        Err("size attribute doesn't exist!".to_string())
                    }
                };
            }
        }

        Err("size attribute doesn't exist!".to_string())
    }
}

pub fn expand_strip(
    children: &[Rc<RefCell<Node>>],
    attributes: &HashMap<String, String>,
) -> proc_macro2::TokenStream {
    let info: StripInfo = attributes.try_into().unwrap();
    let mut expanded = quote! {
        let mut strip_builder = egui_extras::StripBuilder::new(ui);
    };

    let iter: Vec<&Rc<RefCell<Node>>> = if info.direction == StripDirection::BottomUp
        || info.direction == StripDirection::RightToLeft
    {
        children.iter().rev().collect()
    } else {
        children.iter().collect()
    };

    for child in iter.clone() {
        let child_info: StripChildInfo = match child.borrow().get_attributes() {
            Some(child_info) => child_info.try_into().unwrap(),
            None => panic!("No Rust allowed here!"),
        };

        let size_expanded = match child_info.size {
            StripChildSize::Remainder => {
                let size_fn = proc_macro2::Ident::new("remainder", Span::call_site());

                quote! {
                    strip_builder = strip_builder.size(egui_extras::Size::#size_fn());
                }
            }
            StripChildSize::Exact(value) => {
                let size_fn = proc_macro2::Ident::new("exact", Span::call_site());
                let value_literal = proc_macro2::Literal::f32_unsuffixed(value);

                quote! {
                    strip_builder = strip_builder.size(egui_extras::Size::#size_fn(#value_literal));
                }
            }
            StripChildSize::Initial(value) => {
                let size_fn = proc_macro2::Ident::new("initial", Span::call_site());
                let value_literal = proc_macro2::Literal::f32_unsuffixed(value);

                quote! {
                    strip_builder = strip_builder.size(egui_extras::Size::#size_fn(#value_literal));
                }
            }
            StripChildSize::Relative(value) => {
                let size_fn = proc_macro2::Ident::new("relative", Span::call_site());
                let value_literal = proc_macro2::Literal::f32_unsuffixed(value);

                quote! {
                    strip_builder = strip_builder.size(egui_extras::Size::#size_fn(#value_literal));
                }
            }
        };

        expanded.append_all(size_expanded);
    }

    let direction_ident = match info.direction {
        StripDirection::BottomUp => proc_macro2::Ident::new("vertical", Span::call_site()),
        StripDirection::LeftToRight => proc_macro2::Ident::new("horizontal", Span::call_site()),
        StripDirection::RightToLeft => proc_macro2::Ident::new("horizontal", Span::call_site()),
        StripDirection::TopDown => proc_macro2::Ident::new("vertical", Span::call_site()),
    };

    quote_into!(expanded +=
        let strip_response = strip_builder.#direction_ident (|mut strip| {
            #{
                for child in iter {
                    let borrowed_child = child.borrow();

                    let children = match borrowed_child.get_children() {
                        Some(children) => children,
                        None => panic!("No Rust allowed here!"),
                    };

                    if children.is_empty() {
                        quote_into!(expanded += strip.empty();)
                    } else {
                        quote_into!(expanded += strip.cell(|ui| {
                            #{
                                expanded.append_all(crate::expand_node(child));
                            }
                        });)
                    }
                }
            }
        });
    );

    expanded
}
