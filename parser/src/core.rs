use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::from_utf8;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesText, Event};
use quick_xml::reader::Reader;

pub enum Node {
    Panel {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
        code: String,
    },
    Rust {
        parent: Option<Rc<RefCell<Node>>>,
        code: String,
    },
    Border {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
        code: String,
    },
    Grid {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
        code: String,
    },
    Default {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
        code: String,
    },
    Strip {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
        code: String,
    },
}

impl Node {
    pub fn add_node(&mut self, node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        match self {
            Node::Panel { children, .. } => children.push(node.clone()),
            Node::Rust { .. } => {
                panic!("No Children!");
            }
            Node::Border { children, .. } => children.push(node.clone()),
            Node::Grid { children, .. } => children.push(node.clone()),
            Node::Default { children, .. } => children.push(node.clone()),
            Node::Strip { children, .. } => children.push(node.clone()),
        };

        node
    }

    pub fn push_text(&mut self, text: BytesText) {
        match self {
            Node::Panel { code, .. } => code.push_str(from_utf8(&text).unwrap()),
            Node::Rust { code, .. } => code.push_str(from_utf8(&text).unwrap()),
            Node::Border { code, .. } => code.push_str(from_utf8(&text).unwrap()),
            Node::Grid { code, .. } => code.push_str(from_utf8(&text).unwrap()),
            Node::Default { code, .. } => code.push_str(from_utf8(&text).unwrap()),
            Node::Strip { code, .. } => code.push_str(from_utf8(&text).unwrap()),
        };
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        match self {
            Node::Panel { parent, .. } => parent.clone(),
            Node::Rust { parent, .. } => parent.clone(),
            Node::Border { parent, .. } => parent.clone(),
            Node::Grid { parent, .. } => parent.clone(),
            Node::Default { parent, .. } => parent.clone(),
            Node::Strip { parent, .. } => parent.clone(),
        }
    }

    pub fn get_children(&self) -> Option<&Vec<Rc<RefCell<Node>>>> {
        match self {
            Node::Panel { children, .. } => Some(&children),
            Node::Rust { .. } => None,
            Node::Border { children, .. } => Some(&children),
            Node::Grid { children, .. } => Some(&children),
            Node::Default { children, .. } => Some(&children),
            Node::Strip { children, .. } => Some(&children),
        }
    }

    pub fn get_attributes(&self) -> Option<&HashMap<String, Vec<u8>>> {
        match self {
            Node::Panel { attributes, .. } => Some(&attributes),
            Node::Rust { .. } => None,
            Node::Border { attributes, .. } => Some(&attributes),
            Node::Grid { attributes, .. } => Some(&attributes),
            Node::Default { attributes, .. } => Some(&attributes),
            Node::Strip { attributes, .. } => Some(&attributes),
        }
    }

    fn set_attributes(&mut self, attributes: HashMap<String, Vec<u8>>) {
        match self {
            Node::Panel { attributes: a, .. } => *a = attributes,
            Node::Rust { .. } => (),
            Node::Border { attributes: a, .. } => *a = attributes,
            Node::Grid { attributes: a, .. } => *a = attributes,
            Node::Default { attributes: a, .. } => *a = attributes,
            Node::Strip { attributes: a, .. } => *a = attributes,
        }
    }
}

#[derive(Debug)]
pub struct Form {
    #[allow(dead_code)]
    pub nodes: Vec<Rc<RefCell<Node>>>,
    pub attributes: HashMap<String, Vec<u8>>,
}

impl TryFrom<String> for Form {
    type Error = ();

    fn try_from(xml: String) -> Result<Self, Self::Error> {
        let mut reader = Reader::from_str(&xml);
        reader.trim_text(true);

        let mut buf = Vec::new();

        let mut root_starting = false;

        let root: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::Default {
            parent: None,
            children: Vec::new(),
            code: String::new(),
            attributes: HashMap::new(),
        }));

        let mut current_node: Rc<RefCell<Node>> = root.clone();

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,

                Ok(Event::Start(region_start)) => {
                    if !root_starting {
                        if b"Form" == region_start.name().as_ref() {
                            let attributes = prepare_attributes(region_start.attributes());
                            root.borrow_mut().set_attributes(attributes);

                            root_starting = true;
                        }
                    } else {
                        let attributes = prepare_attributes(region_start.attributes());

                        let node = match region_start.name().as_ref() {
                            b"Panel" => Rc::new(RefCell::new(Node::Panel {
                                parent: Some(current_node.clone()),
                                children: Vec::new(),
                                code: String::new(),
                                attributes,
                            })),
                            b"Strip" => Rc::new(RefCell::new(Node::Strip {
                                parent: Some(current_node.clone()),
                                children: Vec::new(),
                                code: String::new(),
                                attributes,
                            })),
                            b"Border" => Rc::new(RefCell::new(Node::Border {
                                parent: Some(current_node.clone()),
                                children: Vec::new(),
                                code: String::new(),
                                attributes,
                            })),
                            b"Grid" => Rc::new(RefCell::new(Node::Grid {
                                parent: Some(current_node.clone()),
                                children: Vec::new(),
                                code: String::new(),
                                attributes,
                            })),
                            b"Rust" => Rc::new(RefCell::new(Node::Rust {
                                parent: Some(current_node.clone()),
                                code: String::new(),
                            })),
                            _ => {
                                panic!("Not a Node {:?}", from_utf8(region_start.name().0).unwrap())
                            }
                        };

                        let new_node = current_node.borrow_mut().add_node(node);
                        current_node = new_node;
                    }
                }
                Ok(Event::Text(text)) => {
                    let text_str = from_utf8(&text).unwrap();

                    current_node
                        .borrow_mut()
                        .add_node(Rc::new(RefCell::new(Node::Rust {
                            parent: Some(current_node.clone()),
                            code: text_str.to_string(),
                        })));
                }
                Ok(Event::End(_)) => {
                    let parent = current_node.borrow_mut().get_parent();

                    if parent.is_some() {
                        current_node = parent.unwrap();
                    }
                }
                _ => (),
            }

            buf.clear();
        }

        let root = root.borrow_mut();

        Ok(Form {
            nodes: root.get_children().unwrap().clone(),
            attributes: root.get_attributes().unwrap().clone(),
        })
    }
}

fn prepare_attributes(attributes: Attributes) -> HashMap<String, Vec<u8>> {
    let mut map = HashMap::new();

    attributes.for_each(|a| {
        let a = a.unwrap();

        map.insert(
            from_utf8(a.key.0).unwrap().to_string(),
            a.value.iter().map(|&x| x).collect::<Vec<u8>>(),
        );
    });

    map
}

pub mod attribute {
    use quote::quote;
    use std::{collections::HashMap, str::FromStr};

    #[derive(Clone)]
    pub enum HybridAttribute<T: Clone> {
        Literal(T),
        DynamicRust(proc_macro2::TokenStream),
    }

    pub fn parse_rust_attribute(
        attributes: &HashMap<String, Vec<u8>>,
        attribute: &str,
    ) -> Result<proc_macro2::TokenStream, String> {
        let code = match attributes.get(attribute) {
            Some(code) => code,
            None => return Err(format!("Attribute {} not found", attribute)),
        };

        if code.is_empty() || code[0] != b'@' {
            return Err("Code must start with @".to_string());
        }

        let code = match std::str::from_utf8(&code[1..]) {
            Ok(code) => code,
            Err(_) => return Err("Failed to parse code".to_string()),
        };

        let stream = match code.parse() {
            Ok(stream) => stream,
            Err(_) => return Err("Failed to parse code".to_string()),
        };

        Ok(stream)
    }

    pub fn parse_optional_rust_attribute(
        attributes: &HashMap<String, Vec<u8>>,
        attribute: &str,
    ) -> Result<Option<proc_macro2::TokenStream>, String> {
        let code = match attributes.get(attribute) {
            Some(code) => code,
            None => return Ok(None),
        };

        if code.is_empty() {
            return Ok(None);
        }

        if code[0] != b'@' {
            return Err("Code must start with @".to_string());
        }

        let code = match std::str::from_utf8(&code[1..]) {
            Ok(code) => code,
            Err(_) => return Err("Failed to parse attribute".to_string()),
        };

        let stream = match code.parse() {
            Ok(stream) => stream,
            Err(_) => return Err("Failed to parse attribute".to_string()),
        };

        Ok(Some(stream))
    }

    pub fn parse_optional_hybrid_attribute<T: FromStr + Into<proc_macro2::TokenStream> + Clone>(
        attributes: &HashMap<String, Vec<u8>>,
        attribute: &str,
    ) -> Result<Option<HybridAttribute<T>>, String> {
        let code = match attributes.get(attribute) {
            Some(code) => code,
            None => return Ok(None),
        };

        if code.is_empty() {
            return Ok(None);
        }

        if code[0] != b'@' {
            let code = match std::str::from_utf8(code) {
                Ok(code) => code,
                Err(_) => return Err("Failed to parse attribute".to_string()),
            };

            let value = match code.parse::<T>() {
                Ok(value) => value,
                Err(_) => return Err("Failed to parse attribute".to_string()),
            };

            return Ok(Some(HybridAttribute::Literal(value)));
        }

        let code = match std::str::from_utf8(&code[1..]) {
            Ok(code) => code,
            Err(_) => return Err("Failed to parse attribute".to_string()),
        };

        match code.parse::<T>() {
            Ok(_) => return Err("Unneeded @ to indicate Rust Code".to_string()),
            Err(_) => (),
        };

        let stream = match code.parse() {
            Ok(stream) => stream,
            Err(_) => {
                return Err("Failed to parse code".to_string());
            }
        };

        Ok(Some(HybridAttribute::DynamicRust(stream)))
    }

    pub fn parse_hybrid_attribute<T: FromStr + Into<proc_macro2::TokenStream> + Clone>(
        attributes: &HashMap<String, Vec<u8>>,
        attribute: &str,
    ) -> Result<HybridAttribute<T>, String> {
        let code = match attributes.get(attribute) {
            Some(code) => code,
            None => return Err(format!("Attribute {} not found", attribute)),
        };

        if code.is_empty() {
            return Err("Attribute is empty".to_string());
        }

        if code[0] != b'@' {
            let code = match std::str::from_utf8(code) {
                Ok(code) => code,
                Err(_) => return Err("Failed to parse attribute".to_string()),
            };

            let value = match code.parse::<T>() {
                Ok(value) => value,
                Err(_) => return Err("Failed to parse attribute".to_string()),
            };

            return Ok(HybridAttribute::Literal(value));
        }

        let code = match std::str::from_utf8(&code[1..]) {
            Ok(code) => code,
            Err(_) => return Err("Failed to parse code".to_string()),
        };

        match code.parse::<T>() {
            Ok(_) => return Err("Unneeded @ to indicate Rust Code".to_string()),
            Err(_) => (),
        };

        let stream = match code.parse() {
            Ok(stream) => stream,
            Err(_) => return Err("Failed to parse code".to_string()),
        };

        Ok(HybridAttribute::DynamicRust(stream))
    }

    pub fn parse_string(
        attributes: &HashMap<String, Vec<u8>>,
        attribute: &str,
    ) -> Result<String, String> {
        let code = match attributes.get(attribute) {
            Some(code) => code,
            None => return Err(format!("Attribute {} not found", attribute)),
        };

        let code = match std::str::from_utf8(code) {
            Ok(code) => code,
            Err(_) => return Err("Failed to parse code".to_string()),
        };

        Ok(code.to_string())
    }

    #[derive(Clone)]
    pub struct AttributeLitF32(pub f32);

    impl FromStr for AttributeLitF32 {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.parse::<f32>() {
                Ok(value) => Ok(AttributeLitF32(value)),
                Err(_) => Err("Failed to parse attribute".to_string()),
            }
        }
    }

    impl Into<proc_macro2::TokenStream> for AttributeLitF32 {
        fn into(self) -> proc_macro2::TokenStream {
            let literal = proc_macro2::Literal::f32_unsuffixed(self.0);

            quote! { #literal }
        }
    }

    #[derive(Clone)]
    pub struct AttributeLitU32(pub u32);

    impl FromStr for AttributeLitU32 {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.parse() {
                Ok(value) => Ok(AttributeLitU32(value)),
                Err(_) => Err("Failed to parse attribute".to_string()),
            }
        }
    }

    impl Into<proc_macro2::TokenStream> for AttributeLitU32 {
        fn into(self) -> proc_macro2::TokenStream {
            let literal = proc_macro2::Literal::u32_unsuffixed(self.0);

            quote! { #literal }
        }
    }

    #[derive(Clone)]
    pub struct AttributeLitString(pub String);

    impl FromStr for AttributeLitString {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(AttributeLitString(s.to_string()))
        }
    }

    impl Into<proc_macro2::TokenStream> for AttributeLitString {
        fn into(self) -> proc_macro2::TokenStream {
            match self.0.parse() {
                Ok(stream) => stream,
                Err(_) => panic!("Failed to parse string"),
            }
        }
    }

    #[derive(Clone)]
    pub struct AttributeLitBool(pub bool);

    impl FromStr for AttributeLitBool {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.parse() {
                Ok(value) => Ok(AttributeLitBool(value)),
                Err(_) => Err("Failed to parse attribute".to_string()),
            }
        }
    }

    impl Into<proc_macro2::TokenStream> for AttributeLitBool {
        fn into(self) -> proc_macro2::TokenStream {
            if self.0 {
                quote! { true }
            } else {
                quote! { false }
            }
        }
    }
}

#[test]
fn test() {
    let xml = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <Form>
        <Strip direction="vertical">
            <Panel pos="exact" arg="100.0">
                <Strip direction="horizontal">
                    <Panel pos="exact" arg="50.0">
                        <UiExecutable ident="ui01"></UiExecutable>
                        <UiExecutable ident="ui02"></UiExecutable>
                    </Panel>
                    <Panel pos="remainder">
                        <UiExecutable ident="ui11"></UiExecutable>
                    </Panel>
                </Strip>
            </Panel>
            <Panel pos="remainder">
                <UiExecutable ident="ui01"></UiExecutable>
            </Panel>
            <Panel pos="relative" arg="0.5">
                <UiExecutable ident="ui01"></UiExecutable>
            </Panel>
        </Strip>
    </Form>"#;

    let form = Form::try_from(xml.to_string()).unwrap();

    println!("{:?}", form);
}
