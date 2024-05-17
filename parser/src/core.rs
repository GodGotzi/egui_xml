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
        attributes: HashMap<String, String>,
        code: String,
    },
    Rust {
        parent: Option<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
        code: String,
    },
    Border {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
        code: String,
    },
    Grid {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
        code: String,
    },
    Default {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
        code: String,
    },
    Strip {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
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

    pub fn get_children(&self) -> &Vec<Rc<RefCell<Node>>> {
        match self {
            Node::Panel { children, .. } => &children,
            Node::Rust { .. } => panic!("No Children!"),
            Node::Border { children, .. } => &children,
            Node::Grid { children, .. } => &children,
            Node::Default { children, .. } => &children,
            Node::Strip { children, .. } => &children,
        }
    }

    pub fn get_attributes(&self) -> &HashMap<String, String> {
        match self {
            Node::Panel { attributes, .. } => &attributes,
            Node::Rust { attributes, .. } => &attributes,
            Node::Border { attributes, .. } => &attributes,
            Node::Grid { attributes, .. } => &attributes,
            Node::Default { attributes, .. } => &attributes,
            Node::Strip { attributes, .. } => &attributes,
        }
    }
}

#[derive(Debug)]
pub struct Form {
    #[allow(dead_code)]
    pub nodes: Vec<Rc<RefCell<Node>>>,
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
                                attributes,
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
                    current_node.borrow_mut().push_text(text);
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
            nodes: root.get_children().clone(),
        })
    }
}

fn prepare_attributes(attributes: Attributes) -> HashMap<String, String> {
    let mut map = HashMap::new();

    attributes.for_each(|a| {
        let a = a.unwrap();

        map.insert(
            from_utf8(a.key.0).unwrap().to_string(),
            from_utf8(a.value.as_ref()).unwrap().to_string(),
        );
    });

    map
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
