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

const SEPARATOR_GAP: f32 = 2.0;

struct StripInfo {
    direction: StripDirection,
    gap: Option<f32>,
    separator: Option<bool>,
}

impl TryFrom<&HashMap<String, String>> for StripInfo {
    type Error = String;

    fn try_from(attributes: &HashMap<String, String>) -> Result<Self, Self::Error> {
        let dir = match attributes.get("direction") {
            Some(dir) => StripDirection::from_str(dir)
                .map_err(|err| format!("StripInfo couldn't be parsed! {:?}", err))?,
            None => return Err("Direction Attribute doesn't exist!".to_string()),
        };

        let mut gap = match attributes.get("gap") {
            Some(gap) => Some(
                gap.parse::<f32>()
                    .map_err(|err| format!("StripInfo couldn't be parsed! {:?}", err))?,
            ),
            None => None,
        };

        let seperator = match attributes.get("separator") {
            Some(seperator) => Some(
                seperator
                    .parse::<bool>()
                    .map_err(|err| format!("StripInfo couldn't be parsed! {:?}", err))?,
            ),
            None => None,
        };

        if seperator.is_some() && gap.is_none() {
            if let Some(value) = gap.clone() {
                gap = Some(value.min(2.0));
            } else {
                gap = Some(2.0);
            }
        }

        Ok(StripInfo {
            direction: dir,
            gap,
            separator: seperator,
        })
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
        let size = match attributes.get("size") {
            Some(size) => size,
            None => return Err("Size Attribute doesn't exist!".to_string()),
        };

        let child_size = StripChildSize::from_str(size)
            .map_err(|err| format!("StripInfo couldn't be parsed! {:?}", err))?;

        return match child_size {
            StripChildSize::Remainder => {
                return Ok(StripChildInfo {
                    size: StripChildSize::Remainder,
                });
            }
            _ => {
                let value_str = match attributes.get("value") {
                    Some(value) => value,
                    None => return Err("Value Attribute doesn't exist!".to_string()),
                };

                let value = value_str
                    .parse::<f32>()
                    .map_err(|err| format!("StripInfo couldn't be parsed! {:?}", err))?;

                match child_size {
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
                }
            }
        };
    }
}

pub fn expand_strip(
    children: &[Rc<RefCell<Node>>],
    attributes: &HashMap<String, String>,
) -> Result<proc_macro2::TokenStream, String> {
    let info: StripInfo = attributes.try_into().unwrap();
    let mut expanded = quote! {
        let mut macro_strip_builder = egui_extras::StripBuilder::new(ui);
    };

    let iter: Vec<&Rc<RefCell<Node>>> = if info.direction == StripDirection::BottomUp
        || info.direction == StripDirection::RightToLeft
    {
        children.iter().rev().collect()
    } else {
        children.iter().collect()
    };

    for (index, child) in iter.iter().enumerate() {
        let child_info: StripChildInfo = match child.borrow().get_attributes() {
            Some(child_info) => child_info.try_into()?,
            None => return Err("No Rust allowed here!".to_string()),
        };

        let size_expanded = match child_info.size {
            StripChildSize::Remainder => {
                let size_fn = proc_macro2::Ident::new("remainder", Span::call_site());

                quote! {
                    macro_strip_builder = macro_strip_builder.size(egui_extras::Size::#size_fn());
                }
            }
            StripChildSize::Exact(value) => {
                let size_fn = proc_macro2::Ident::new("exact", Span::call_site());
                let value_literal = proc_macro2::Literal::f32_unsuffixed(value);

                quote! {
                    macro_strip_builder = macro_strip_builder.size(egui_extras::Size::#size_fn(#value_literal));
                }
            }
            StripChildSize::Initial(value) => {
                let size_fn = proc_macro2::Ident::new("initial", Span::call_site());
                let value_literal = proc_macro2::Literal::f32_unsuffixed(value);

                quote! {
                    macro_strip_builder = macro_strip_builder.size(egui_extras::Size::#size_fn(#value_literal));
                }
            }
            StripChildSize::Relative(value) => {
                let size_fn = proc_macro2::Ident::new("relative", Span::call_site());
                let value_literal = proc_macro2::Literal::f32_unsuffixed(value);

                quote! {
                    macro_strip_builder = macro_strip_builder.size(egui_extras::Size::#size_fn(#value_literal));
                }
            }
        };

        expanded.append_all(size_expanded);

        if iter.len() - 1 != index {
            if info.gap.is_some() {
                let gap = info.gap.unwrap_or(SEPARATOR_GAP);
                let gap_literal = proc_macro2::Literal::f32_unsuffixed(gap);

                let gap_fn = proc_macro2::Ident::new("exact", Span::call_site());

                expanded.append_all(quote! {
                    macro_strip_builder = macro_strip_builder.size(egui_extras::Size::#gap_fn(#gap_literal));
                });
            }
        }
    }

    let direction_ident = match info.direction {
        StripDirection::BottomUp => proc_macro2::Ident::new("vertical", Span::call_site()),
        StripDirection::LeftToRight => proc_macro2::Ident::new("horizontal", Span::call_site()),
        StripDirection::RightToLeft => proc_macro2::Ident::new("horizontal", Span::call_site()),
        StripDirection::TopDown => proc_macro2::Ident::new("vertical", Span::call_site()),
    };

    quote_into!(expanded +=
        let macro_strip_response = macro_strip_builder.#direction_ident (|mut strip| {
            #{
                for (index, child) in iter.iter().enumerate() {
                    let borrowed_child = child.borrow();

                    let children = match borrowed_child.get_children() {
                        Some(children) => children,
                        None => return Err("No Rust allowed here!".to_string()),
                    };

                    if children.is_empty() {
                        quote_into!(expanded += strip.empty();)
                    } else {
                        quote_into!(expanded += strip.cell(|ui| {
                            #{
                                expanded.append_all(crate::expand_node(child)?);
                            }
                        });)
                    }

                    if iter.len() - 1 != index {
                        if info.gap.is_some() {
                            if info.separator.unwrap_or(false) {
                                quote_into!(expanded += strip.cell(|ui| {
                                    ui.separator();
                                });)
                            } else {
                                quote_into!(expanded += strip.empty();)
                            }
                        }
                    }
                }
            }
        });
    );

    Ok(expanded)
}
