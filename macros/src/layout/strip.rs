use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    str::{from_utf8, FromStr},
};

use parser::{
    attribute::{
        parse_optional_hybrid_attribute, AttributeLitBool, AttributeLitF32, HybridAttribute,
    },
    Node,
};
use proc_macro2::Span;
use quote::{quote, TokenStreamExt};

use quote_into::quote_into;

use strum_macros::EnumString;

use crate::XMLContext;

use parser::attribute::{parse_hybrid_attribute, parse_optional_rust_attribute, parse_string};

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

    // Rust Token Stream
    separator: Option<HybridAttribute<AttributeLitBool>>,
    gap: Option<HybridAttribute<AttributeLitF32>>,
    ui: proc_macro2::TokenStream,
}

impl TryFrom<&HashMap<String, Vec<u8>>> for StripInfo {
    type Error = String;

    fn try_from(attributes: &HashMap<String, Vec<u8>>) -> Result<Self, Self::Error> {
        let direction = StripDirection::from_str(&parse_string(attributes, "direction")?)
            .map_err(|err| format!("StripInfo couldn't be parsed! {:?}", err))?;

        let gap = parse_optional_hybrid_attribute::<AttributeLitF32>(attributes, "gap")?;

        let separator =
            parse_optional_hybrid_attribute::<AttributeLitBool>(attributes, "separator")?;

        let ui = parse_optional_rust_attribute(attributes, "ui")?.unwrap_or(quote! { ui });

        Ok(StripInfo {
            direction,
            gap,
            separator,
            ui,
        })
    }
}

#[derive(EnumString)]
enum StripChildSize {
    #[strum(serialize = "Remainder", serialize = "remainder")]
    Remainder,
    #[strum(serialize = "Exact", serialize = "exact")]
    Exact(proc_macro2::TokenStream),
    #[strum(serialize = "Initial", serialize = "initial")]
    Initial(proc_macro2::TokenStream),
    #[strum(serialize = "Relative", serialize = "relative")]
    Relative(proc_macro2::TokenStream),
}

struct StripChildInfo {
    size: StripChildSize,
}

impl TryFrom<&HashMap<String, Vec<u8>>> for StripChildInfo {
    type Error = String;

    fn try_from(attributes: &HashMap<String, Vec<u8>>) -> Result<Self, Self::Error> {
        let size = match attributes.get("size") {
            Some(size) => from_utf8(size).unwrap(),
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
                let hybrid_attribute =
                    parse_hybrid_attribute::<AttributeLitF32>(attributes, "value")?;

                let stream = match hybrid_attribute {
                    HybridAttribute::Literal(value) => value.into(),
                    HybridAttribute::DynamicRust(stream) => stream,
                };

                match child_size {
                    StripChildSize::Exact(_) => Ok(StripChildInfo {
                        size: StripChildSize::Exact(stream),
                    }),
                    StripChildSize::Initial(_) => Ok(StripChildInfo {
                        size: StripChildSize::Initial(stream),
                    }),
                    StripChildSize::Relative(_) => Ok(StripChildInfo {
                        size: StripChildSize::Relative(stream),
                    }),
                    _ => panic!("Why you here!"),
                }
            }
        };
    }
}

pub fn expand_strip(
    strip: &Rc<RefCell<Node>>,
    ctx: &XMLContext,
) -> Result<proc_macro2::TokenStream, String> {
    let children_borrow = strip.borrow();
    let children = children_borrow.get_children().unwrap();

    let attributes_borrow = strip.borrow();
    let attributes = attributes_borrow.get_attributes().unwrap();

    let info: StripInfo = attributes.try_into()?;

    let ui_var = info.ui;

    let mut expanded = quote! {
        let mut macro_strip_builder = egui_extras::StripBuilder::new(#ui_var);
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
            StripChildSize::Exact(stream) => {
                let size_fn = proc_macro2::Ident::new("exact", Span::call_site());

                quote! {
                    macro_strip_builder = macro_strip_builder.size(egui_extras::Size::#size_fn(#stream));
                }
            }
            StripChildSize::Initial(stream) => {
                let size_fn = proc_macro2::Ident::new("initial", Span::call_site());

                quote! {
                    macro_strip_builder = macro_strip_builder.size(egui_extras::Size::#size_fn(#stream));
                }
            }
            StripChildSize::Relative(stream) => {
                let size_fn = proc_macro2::Ident::new("relative", Span::call_site());

                quote! {
                    macro_strip_builder = macro_strip_builder.size(egui_extras::Size::#size_fn(#stream));
                }
            }
        };

        expanded.append_all(size_expanded);

        if iter.len() - 1 != index {
            if let Some(gap) = info.gap.clone() {
                let gap_stream = match gap {
                    HybridAttribute::Literal(value) => value.into(),
                    HybridAttribute::DynamicRust(stream) => stream,
                };

                let gap_fn = proc_macro2::Ident::new("exact", Span::call_site());

                expanded.append_all(quote! {
                    macro_strip_builder = macro_strip_builder.size(egui_extras::Size::#gap_fn(#gap_stream));
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
                                expanded.append_all(crate::expand_node(child, &ctx)?);
                            }
                        });)
                    }

                    if iter.len() - 1 != index {
                        if info.gap.is_some() {
                            if let Some(sep) = info.separator.clone() {
                                match sep {
                                    HybridAttribute::Literal(value) => {
                                        if value.0 {
                                            quote_into!(expanded +=
                                                strip.cell(|ui| {
                                                    ui.separator();
                                                });
                                            )
                                        } else {
                                            quote_into!(expanded +=
                                                strip.empty();
                                            )
                                        }
                                    }
                                    HybridAttribute::DynamicRust(stream) => {
                                        quote_into!(expanded +=
                                            if #stream {
                                                strip.cell(|ui| {
                                                    ui.separator();
                                                });
                                            } else {
                                                strip.empty();
                                            }
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    );

    Ok(expanded)
}
