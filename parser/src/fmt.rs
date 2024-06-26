use crate::core::Node;

fn calculate_nested_amount(node: &Node) -> usize {
    if node.get_parent().is_none() {
        return 0;
    }

    let mut amount = 1;

    let mut current_node = node.get_parent().unwrap();

    while current_node.borrow().get_parent().is_some() {
        amount += 1;

        let parent = current_node.borrow().get_parent().unwrap();
        current_node = parent;
    }
    // nvim 

    amount
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let amount = calculate_nested_amount(&self);

        write!(f, "\n\t{:indent$}", "", indent = amount * 4)?;
        write!(f, "\n\t{:indent$}", "", indent = amount * 4)?;

        match self {
            Node::Panel {
                children,
                attributes,
                ..
            } => {
                write!(f, "<Panel {:?}> ", attributes)?;
                for child in children {
                    child.borrow().fmt(f).unwrap();
                }
                write!(f, "\n\t{:indent$}", "", indent = amount * 4)?;
                writeln!(f, "</Panel>")?;
            }
            Node::Rust { code, .. } => {
                write!(f, "{:?}", code)?;
            }
            Node::Border { children, .. } => {
                write!(f, "<Border>")?;
                for child in children {
                    child.borrow().fmt(f).unwrap();
                }
                write!(f, "\n\t{:indent$}", "", indent = amount * 4)?;
                writeln!(f, "</Border>")?;
            }
            Node::Grid { children, .. } => {
                write!(f, "<Grid>")?;
                for child in children {
                    child.borrow().fmt(f).unwrap();
                }
                write!(f, "\n\t{:indent$}", "", indent = amount * 4)?;
                writeln!(f, "</Grid>")?;
            }
            Node::Default { children, .. } => {
                write!(f, "<Default>")?;
                for child in children {
                    child.borrow().fmt(f).unwrap();
                }
                write!(f, "\n\t{:indent$}", "", indent = amount * 4)?;
                writeln!(f, "</Default>")?;
            }
            Node::Strip {
                children,
                attributes,
                ..
            } => {
                write!(f, "<Strip {:?}>", attributes)?;
                for child in children {
                    child.borrow().fmt(f).unwrap();
                }
                write!(f, "\n\t{:indent$}", "", indent = amount * 4)?;
                writeln!(f, "</Strip>")?;
            }
        }

        Ok(())
    }
}

mod test {

    #[test]
    fn test_fmt() {
        use crate::XMLForm;

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

        let debug_str = format!("{:?}", form)
            .replace("\n", "")
            .replace("\t", "")
            .replace(" ", "");

        println!("{:?}", debug_str);
    }
}
