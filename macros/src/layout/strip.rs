use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    str::{from_utf8, FromStr},
};

use egui_xml_parser::{
    attribute::{parse_optional_hybrid_attribute, AttributeBool, AttributeF32, HybridAttribute},
    Node,
};
use proc_macro2::Span;
use quote::{quote, TokenStreamExt};

use quote_into::quote_into;

use strum_macros::EnumString;

use crate::XMLContext;

use egui_xml_parser::attribute::{
    parse_hybrid_attribute, parse_optional_rust_attribute, parse_string,
};

#[derive(PartialEq, Eq, EnumString)]
enum DirectionBlueprint {
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

struct StripBlueprint {
    direction: DirectionBlueprint,

    // Rust Token Stream
    gap: Option<HybridAttribute<AttributeF32>>,
    separator: HybridAttribute<AttributeBool>,
    ui: proc_macro2::TokenStream,
}

impl TryFrom<&HashMap<String, Vec<u8>>> for StripBlueprint {
    type Error = String;

    fn try_from(attributes: &HashMap<String, Vec<u8>>) -> Result<Self, Self::Error> {
        let direction = DirectionBlueprint::from_str(&parse_string(attributes, "direction")?)
            .map_err(|err| format!("StripInfo couldn't be parsed! {:?}", err))?;

        let gap = parse_optional_hybrid_attribute::<AttributeF32>(attributes, "gap")?;

        let separator = parse_optional_hybrid_attribute::<AttributeBool>(attributes, "separator")?
            .unwrap_or(HybridAttribute::Literal(AttributeBool(false)));

        let ui = parse_optional_rust_attribute(attributes, "ui")?.unwrap_or(quote! { ui });

        Ok(StripBlueprint {
            direction,
            gap,
            separator,
            ui,
        })
    }
}

#[derive(PartialEq, Eq, EnumString)]
enum SizeType {
    #[strum(serialize = "Remainder", serialize = "remainder", serialize = "rem")]
    Remainder,
    #[strum(serialize = "Exact", serialize = "exact", serialize = "ex")]
    Exact,
    #[strum(serialize = "Relative", serialize = "relative", serialize = "rel")]
    Relative,
}

pub enum SizeBlueprint {
    Remainder {
        min: Option<HybridAttribute<AttributeF32>>,
        max: Option<HybridAttribute<AttributeF32>>,
    },
    Exact {
        value: HybridAttribute<AttributeF32>,

        min: Option<HybridAttribute<AttributeF32>>,
        max: Option<HybridAttribute<AttributeF32>>,
    },
    Relative {
        value: HybridAttribute<AttributeF32>,

        min: Option<HybridAttribute<AttributeF32>>,
        max: Option<HybridAttribute<AttributeF32>>,
    },
}

impl TryFrom<&HashMap<String, Vec<u8>>> for SizeBlueprint {
    type Error = String;

    fn try_from(attributes: &HashMap<String, Vec<u8>>) -> Result<Self, Self::Error> {
        let size = match attributes.get("size") {
            Some(size) => from_utf8(size).unwrap(),
            None => return Err("Size Attribute doesn't exist!".to_string()),
        };

        let size_type = SizeType::from_str(size)
            .map_err(|err| format!("StripInfo couldn't be parsed! {:?}", err))?;

        let min = parse_optional_hybrid_attribute::<AttributeF32>(attributes, "min")?;

        let max = parse_optional_hybrid_attribute::<AttributeF32>(attributes, "max")?;

        return match size_type {
            SizeType::Remainder => {
                return Ok(SizeBlueprint::Remainder { min, max });
            }
            _ => {
                let hybrid_attribute = parse_hybrid_attribute::<AttributeF32>(attributes, "value")?;

                match size_type {
                    SizeType::Exact => Ok(SizeBlueprint::Exact {
                        value: hybrid_attribute,
                        min,
                        max,
                    }),
                    SizeType::Relative => Ok(SizeBlueprint::Relative {
                        value: hybrid_attribute,
                        min,
                        max,
                    }),
                    _ => panic!("Why you here!"),
                }
            }
        };
    }
}

impl Into<proc_macro2::TokenStream> for SizeBlueprint {
    fn into(self) -> proc_macro2::TokenStream {
        let min_fn = proc_macro2::Ident::new("at_least", Span::call_site());
        let max_fn = proc_macro2::Ident::new("at_most", Span::call_site());

        match self {
            SizeBlueprint::Remainder { min, max } => {
                let mut expanded = quote! { egui_extras::Size::remainder() };

                if let Some(min) = min {
                    let stream: proc_macro2::TokenStream = min.into();

                    expanded.append_all(quote! {.#min_fn(#stream)});
                }

                if let Some(max) = max {
                    let stream: proc_macro2::TokenStream = max.into();

                    expanded.append_all(quote! {.#max_fn(#stream)});
                }

                quote! {
                    macro_strip_builder = macro_strip_builder.size(#expanded);
                }
            }
            SizeBlueprint::Exact { value, min, max } => {
                let stream: proc_macro2::TokenStream = value.into();

                let mut expanded = quote! { egui_extras::Size::exact(#stream) };

                if let Some(min) = min {
                    let stream: proc_macro2::TokenStream = min.into();

                    expanded.append_all(quote! {.#min_fn(#stream)});
                }

                if let Some(max) = max {
                    let stream: proc_macro2::TokenStream = max.into();

                    expanded.append_all(quote! {.#max_fn(#stream)});
                }

                quote! {
                    macro_strip_builder = macro_strip_builder.size(#expanded);
                }
            }
            SizeBlueprint::Relative { value, min, max } => {
                let stream: proc_macro2::TokenStream = value.into();

                let mut expanded = quote! { egui_extras::Size::relative(#stream) };

                if let Some(min) = min {
                    let stream: proc_macro2::TokenStream = min.into();

                    expanded.append_all(quote! {.#min_fn(#stream)});
                }

                if let Some(max) = max {
                    let stream: proc_macro2::TokenStream = max.into();

                    expanded.append_all(quote! {.#max_fn(#stream)});
                }

                quote! {
                    macro_strip_builder = macro_strip_builder.size(#expanded);
                }
            }
        }
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

    let info: StripBlueprint = attributes.try_into()?;

    let ui_var = info.ui;

    let mut expanded = quote! {
        let mut macro_strip_builder = egui_extras::StripBuilder::new(#ui_var);
    };

    let iter: Vec<&Rc<RefCell<Node>>> = if info.direction == DirectionBlueprint::BottomUp
        || info.direction == DirectionBlueprint::RightToLeft
    {
        children.iter().rev().collect()
    } else {
        children.iter().collect()
    };

    for (index, child) in iter.iter().enumerate() {
        let child_blueprint: SizeBlueprint = match child.borrow().get_attributes() {
            Some(attributes) => attributes.try_into()?,
            None => return Err("No Rust allowed here!".to_string()),
        };

        let size_expanded: proc_macro2::TokenStream = child_blueprint.into();

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
        DirectionBlueprint::BottomUp => proc_macro2::Ident::new("vertical", Span::call_site()),
        DirectionBlueprint::LeftToRight => proc_macro2::Ident::new("horizontal", Span::call_site()),
        DirectionBlueprint::RightToLeft => proc_macro2::Ident::new("horizontal", Span::call_site()),
        DirectionBlueprint::TopDown => proc_macro2::Ident::new("vertical", Span::call_site()),
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
                            match &info.separator {
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
        });
    );

    Ok(expanded)
}
