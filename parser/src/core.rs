use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::from_utf8;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

#[derive()]
pub enum Node {
    Panel {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
    },
    Rust {
        parent: Option<Rc<RefCell<Node>>>,
        code: String,
    },
    Border {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
    },
    Grid {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
    },
    Default {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
    },
    Strip {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, Vec<u8>>,
    },
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Node::Panel {
                    children: children1,
                    attributes: attributes1,
                    ..
                },
                Node::Panel {
                    children: children2,
                    attributes: attributes2,
                    ..
                },
            ) => children1 == children2 && attributes1 == attributes2,
            (Node::Rust { code: code1, .. }, Node::Rust { code: code2, .. }) => code1 == code2,
            (
                Node::Border {
                    children: children1,
                    attributes: attributes1,
                    ..
                },
                Node::Border {
                    children: children2,
                    attributes: attributes2,
                    ..
                },
            ) => children1 == children2 && attributes1 == attributes2,
            (
                Node::Grid {
                    children: children1,
                    attributes: attributes1,
                    ..
                },
                Node::Grid {
                    children: children2,
                    attributes: attributes2,
                    ..
                },
            ) => children1 == children2 && attributes1 == attributes2,
            (
                Node::Default {
                    children: children1,
                    attributes: attributes1,
                    ..
                },
                Node::Default {
                    children: children2,
                    attributes: attributes2,
                    ..
                },
            ) => children1 == children2 && attributes1 == attributes2,
            (
                Node::Strip {
                    children: children1,
                    attributes: attributes1,
                    ..
                },
                Node::Strip {
                    children: children2,
                    attributes: attributes2,
                    ..
                },
            ) => children1 == children2 && attributes1 == attributes2,
            _ => false,
        }
    }
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

    #[allow(dead_code)]
    pub(crate) fn get_parent_mut(&mut self) -> &mut Option<Rc<RefCell<Node>>> {
        match self {
            Node::Panel { parent, .. } => parent,
            Node::Rust { parent, .. } => parent,
            Node::Border { parent, .. } => parent,
            Node::Grid { parent, .. } => parent,
            Node::Default { parent, .. } => parent,
            Node::Strip { parent, .. } => parent,
        }
    }
}

#[derive(Debug)]
pub struct XMLForm {
    pub root: Rc<RefCell<Node>>,
}

impl PartialEq for XMLForm {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl TryFrom<String> for XMLForm {
    type Error = String;

    fn try_from(xml: String) -> Result<Self, Self::Error> {
        let mut reader = Reader::from_str(&xml);
        reader.trim_text(true);

        let mut buf = Vec::new();

        let root: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::Default {
            parent: None,
            children: Vec::new(),
            attributes: HashMap::new(),
        }));

        let mut current_node: Rc<RefCell<Node>> = root.clone();

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,

                Ok(Event::Start(region_start)) => {
                    let attributes = prepare_attributes(region_start.attributes());

                    let node = match region_start.name().as_ref() {
                        b"Panel" => Rc::new(RefCell::new(Node::Panel {
                            parent: Some(current_node.clone()),
                            children: Vec::new(),
                            attributes,
                        })),
                        b"Strip" => Rc::new(RefCell::new(Node::Strip {
                            parent: Some(current_node.clone()),
                            children: Vec::new(),
                            attributes,
                        })),
                        b"Border" => Rc::new(RefCell::new(Node::Border {
                            parent: Some(current_node.clone()),
                            children: Vec::new(),
                            attributes,
                        })),
                        b"Grid" => Rc::new(RefCell::new(Node::Grid {
                            parent: Some(current_node.clone()),
                            children: Vec::new(),
                            attributes,
                        })),
                        b"Rust" => Rc::new(RefCell::new(Node::Rust {
                            parent: Some(current_node.clone()),
                            code: "".to_string(),
                        })),
                        _ => {
                            panic!("Not a Node {:?}", from_utf8(region_start.name().0).unwrap())
                        }
                    };

                    let new_node = current_node.borrow_mut().add_node(node);
                    current_node = new_node;
                }
                Ok(Event::Text(text)) => {
                    let text_str = from_utf8(&text).unwrap();

                    // current_node.borrow_mut().push_text(&text);

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

        Ok(XMLForm { root })
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

    impl<T: Into<proc_macro2::TokenStream> + Clone> Into<proc_macro2::TokenStream>
        for HybridAttribute<T>
    {
        fn into(self) -> proc_macro2::TokenStream {
            match self {
                HybridAttribute::Literal(value) => value.into(),
                HybridAttribute::DynamicRust(stream) => stream,
            }
        }
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
    pub struct AttributeF32(pub f32);

    impl FromStr for AttributeF32 {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.parse::<f32>() {
                Ok(value) => Ok(AttributeF32(value)),
                Err(_) => Err("Failed to parse attribute".to_string()),
            }
        }
    }

    impl Into<proc_macro2::TokenStream> for AttributeF32 {
        fn into(self) -> proc_macro2::TokenStream {
            let literal = proc_macro2::Literal::f32_unsuffixed(self.0);

            quote! { #literal }
        }
    }

    #[derive(Clone)]
    pub struct AttributeU32(pub u32);

    impl FromStr for AttributeU32 {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.parse() {
                Ok(value) => Ok(AttributeU32(value)),
                Err(_) => Err("Failed to parse attribute".to_string()),
            }
        }
    }

    impl Into<proc_macro2::TokenStream> for AttributeU32 {
        fn into(self) -> proc_macro2::TokenStream {
            let literal = proc_macro2::Literal::u32_unsuffixed(self.0);

            quote! { #literal }
        }
    }

    #[derive(Clone)]
    pub struct AttributeString(pub String);

    impl FromStr for AttributeString {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(AttributeString(s.to_string()))
        }
    }

    impl Into<proc_macro2::TokenStream> for AttributeString {
        fn into(self) -> proc_macro2::TokenStream {
            match self.0.parse() {
                Ok(stream) => stream,
                Err(_) => panic!("Failed to parse string"),
            }
        }
    }

    #[derive(Clone)]
    pub struct AttributeBool(pub bool);

    impl FromStr for AttributeBool {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.parse() {
                Ok(value) => Ok(AttributeBool(value)),
                Err(_) => Err("Failed to parse attribute".to_string()),
            }
        }
    }

    impl Into<proc_macro2::TokenStream> for AttributeBool {
        fn into(self) -> proc_macro2::TokenStream {
            if self.0 {
                quote! { true }
            } else {
                quote! { false }
            }
        }
    }

    #[derive(Clone)]
    pub struct AttributeSizeType(pub String);
}

mod test {
    #[test]
    fn test() {
        use super::{Node, XMLForm};
        use std::cell::RefCell;
        use std::collections::HashMap;
        use std::rc::Rc;

        let xml = r#"
        <Form>
            <Strip direction="south">
                <Panel size="relative" value="0.4">
                    <Strip direction="east">
                        <Panel size="exact" value="250.0">
                            if ui.button("Hi I am a button!").clicked() {println!("Button clicked!");}
                        </Panel>
                        <Panel size="remainder">
                            ui.label("Hello from XML!");
                        </Panel>
                    </Strip>
                </Panel>
                <Panel size="remainder">
                    ui.label("Hello from XML!");
                </Panel>
            </Strip>
        </Form>
        "#;

        let form = XMLForm::try_from(xml.to_string()).unwrap();

        let root = Rc::new(RefCell::new(Node::Default {
            parent: None,
            children: vec![],
            attributes: HashMap::new(),
        }));

        let strip = Rc::new(RefCell::new(Node::Strip {
            parent: None,
            children: vec![
                Rc::new(RefCell::new(Node::Panel {
                    parent: None,
                    children: vec![Rc::new(RefCell::new(Node::Strip {
                        parent: None,
                        children: vec![
                            Rc::new(RefCell::new(Node::Panel {
                                parent: None,
                                children: vec![Rc::new(RefCell::new(Node::Rust {
                                    parent: None,
                                    code: "if ui.button(\"Hi I am a button!\").clicked() {println!(\"Button clicked!\");}"
                                        .to_string(),
                                }))],
                                attributes: {
                                    let mut map = HashMap::new();
                                    map.insert("size".to_string(), "exact".as_bytes().to_vec());
                                    map.insert("value".to_string(), "250.0".as_bytes().to_vec());
                                    map
                                },
                            })),
                            Rc::new(RefCell::new(Node::Panel {
                                parent: None,
                                children: vec![Rc::new(RefCell::new(Node::Rust {
                                    parent: None,
                                    code: "ui.label(\"Hello from XML!\");".to_string(),
                                }))],
                                attributes: {
                                    let mut map = HashMap::new();
                                    map.insert("size".to_string(), "remainder".as_bytes().to_vec());
                                    map
                                },
                            })),
                        ],
                        attributes: {
                            let mut map = HashMap::new();
                            map.insert("direction".to_string(), "east".as_bytes().to_vec());
                            map
                        },
                    }))],
                    attributes: {
                        let mut map = HashMap::new();
                        map.insert("size".to_string(), "relative".as_bytes().to_vec());
                        map.insert("value".to_string(), "0.4".as_bytes().to_vec());
                        map
                    },
                })),
                Rc::new(RefCell::new(Node::Panel {
                    parent: None,
                    children: vec![Rc::new(RefCell::new(Node::Rust {
                        parent: None,
                        code: "ui.label(\"Hello from XML!\");".to_string(),
                    }))],
                    attributes: {
                        let mut map = HashMap::new();
                        map.insert("size".to_string(), "remainder".as_bytes().to_vec());
                        map
                    },
                })),
            ],
            attributes: {
                let mut map = HashMap::new();
                map.insert("direction".to_string(), "south".as_bytes().to_vec());
                map
            },
        }));

        fn set_parent_recursive(node: Rc<RefCell<Node>>, parent: Option<Rc<RefCell<Node>>>) {
            *node.borrow_mut().get_parent_mut() = parent;

            if let Some(children) = node.borrow_mut().get_children() {
                children.iter().for_each(|child| {
                    set_parent_recursive(child.clone(), Some(node.clone()));
                });
            }
        }

        set_parent_recursive(strip.clone(), None);

        root.borrow_mut().add_node(strip);

        let eq_form = XMLForm { root };

        assert_eq!(form, eq_form);
    }
}
