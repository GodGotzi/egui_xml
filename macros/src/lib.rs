extern crate proc_macro;

use std::collections::HashMap;

use egui::{Align, Direction};
use proc_macro::TokenStream;

// note: to use serde, the feature needs to be enabled
// run example with:
//    cargo run --example read_nodes_serde --features="serialize"

// note: to use serde, the feature needs to be enabled
// run example with:
//    cargo run --example read_nodes_serde --features="serialize"

use quick_xml::de::from_str;
use serde::{
    de::{value::MapAccessDeserializer, Error, MapAccess, Visitor},
    Deserialize,
};
use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default)]
struct Translation {
    #[serde(rename = "@Tag")]
    tag: String,
    #[serde(rename = "@Language")]
    lang: String,
    #[serde(rename = "$text")]
    text: String,
}

const XML: &str = r#"
<Form>
    <model>
        <elem type="foo">
            <a>1</a>
            <subfoo>
                <a1>2</a1>
                <a2>42</a2>
                <a3>1337</a3>
            </subfoo>
        </elem>
        <elem type="bar">
            <b>22</b>
        </elem>
    </model>
</Form>
"#;

#[derive(Debug, PartialEq)]
struct Form {
    root: Node,
}

#[derive(Debug, PartialEq)]
enum Node {
    Panel {
        children: Vec<Node>,
        args: HashMap<String, String>,
    },
    Div {
        inner: InnerLayoutDiv,
    },
    UiExecutable {
        variable: String,
    },
}

#[derive(Debug, PartialEq)]
enum InnerLayoutDiv {
    Border {
        children: Vec<Node>,
    },
    Grid {
        columns: usize,
        rows: usize,
        horizontal_spacing: f32,
        vertical_spacing: f32,
        children: Vec<Node>,
    },
    Default {
        children: Vec<Node>,
    },
    Strip {
        children: Vec<Node>,
        direction: Direction,
    },
}

impl<'de> serde::Deserialize<'de> for Form {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FormVisitor;

        impl<'de> Visitor<'de> for FormVisitor {
            type Value = Form;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an object with a `type` field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Form, A::Error>
            where
                A: MapAccess<'de>,
            {
                // let mut children = Vec::new();

                // println!("map: {:?}", map.next_key::<String>());
                // println!("map: {:?}", map.next_value::<String>());

                while let Ok(Some(key)) = map.next_key::<String>() {
                    println!("key: {:?}", key);
                }

                return Ok(Form {
                    root: Node::Div {
                        inner: InnerLayoutDiv::Default {
                            children: Vec::new(),
                        },
                    },
                });
                // Err(Error::custom("expected `type` attribute"))
            }
        }
        deserializer.deserialize_map(FormVisitor)
    }
}

mod test {
    use quick_xml::de::from_str;

    use crate::{Form, XML};

    #[test]
    fn test_de() {
        let form: Form = from_str(XML).unwrap();
        println!("{:?}", form);
    }
}

#[proc_macro]
pub fn form(input: TokenStream) -> TokenStream {
    input
}
