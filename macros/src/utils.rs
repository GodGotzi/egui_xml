use std::{cell::RefCell, rc::Rc};

use parser::Node;

pub struct ExposeNode(pub(crate) Rc<RefCell<Node>>);
