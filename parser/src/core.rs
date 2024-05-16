use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::from_utf8;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

#[derive(Debug)]
pub struct Form {
    nodes: Vec<Rc<RefCell<Node>>>,
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
                            println!("Root Started");
                        }
                    } else {
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
                            _ => todo!(),
                        };

                        let new_node = current_node.borrow_mut().add_node(node);
                        current_node = new_node;
                    }
                }
                Ok(Event::End(_)) => {
                    let parent = current_node.borrow_mut().get_parent();

                    println!("Return to Parent");

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

pub enum Node {
    Panel {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
    },
    UiExecutable {
        parent: Option<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
    },
    Border {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
    },
    Grid {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
    },
    Default {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
    },
    Strip {
        parent: Option<Rc<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
        attributes: HashMap<String, String>,
    },
}

impl Node {
    pub fn add_node(&mut self, node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        match self {
            Node::Panel { children, .. } => children.push(node.clone()),
            Node::UiExecutable { .. } => {
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
            Node::UiExecutable { parent, .. } => parent.clone(),
            Node::Border { parent, .. } => parent.clone(),
            Node::Grid { parent, .. } => parent.clone(),
            Node::Default { parent, .. } => parent.clone(),
            Node::Strip { parent, .. } => parent.clone(),
        }
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        match self {
            Node::Panel { children, .. } => children.clone(),
            Node::UiExecutable { .. } => Vec::new(),
            Node::Border { children, .. } => children.clone(),
            Node::Grid { children, .. } => children.clone(),
            Node::Default { children, .. } => children.clone(),
            Node::Strip { children, .. } => children.clone(),
        }
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
                    </Panel>
                    <Panel pos="remainder">
                    </Panel>
                </Strip>
            </Panel>
            <Panel pos="remainder">
            </Panel>
            <Panel pos="relative" arg="0.5">
            </Panel>
        </Strip>
    </Form>"#;

    let form = Form::try_from(xml.to_string()).unwrap();

    println!("{:?}", form);
}
