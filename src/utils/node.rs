use super::batch::Batch;

use crate::prelude::*;
use crate::utils::Grid;
use std::cell::RefCell;
use std::rc::Rc;

//Single Node

#[derive(Clone)]
pub struct Node {
    pub parent: Option<String>,
    pub grid: Option<Grid>,
    pub area: Option<String>,

    pub body: Rc<RefCell<dyn ElementCore>>,
    pub id: String,
}

impl Node {
    pub fn get_grid(&mut self) -> Grid {
        self.grid.unwrap()
    }
}

//Node tree
pub struct NodeWalker {
    tree: Vec<Node>,
}

impl NodeWalker {
    pub fn create() -> NodeWalker {
        NodeWalker { tree: Vec::new() }
    }

    pub fn add(&mut self, node: Node) {
        if node.parent.is_some() {
            let mut parent = self.find(node.clone().parent.unwrap());
            let result = parent.get_grid().get_position(&node.area.clone().unwrap());

            node.body
                .borrow_mut()
                .x(parent.body.borrow_mut().get_x() + result.x);
            node.body
                .borrow_mut()
                .y(parent.body.borrow_mut().get_y() + result.y);

            if let Some(el) = node.body.borrow_mut().is_resizable() {
                el.width(result.width);
                el.height(result.height);
            }

            node.body.borrow_mut().build();
            self.tree.push(node.clone());
        } else {
            self.tree.push(node);
        }
    }

    pub fn find(&mut self, id: String) -> Node {
        self.tree
            .iter()
            .find(|node| node.id == id)
            .expect("No Node found")
            .clone()
    }

    pub fn get_batch(&mut self) -> Batch {
        let mut batch = Batch::new();

        for node in self.tree.iter() {
            batch.add_mesh(&node.body.borrow_mut().mesh())
        }

        batch
    }
}
