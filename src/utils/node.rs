use super::batch::Batch;

use crate::prelude::*;


//Single Node

#[derive(Clone)]
pub struct Node<T: ElementCore> {
    pub parent: Option<String>,
    pub grid: Option<Grid>,
    pub area: Option<String>,

    pub body: T,
    pub id: String,
}

impl<T: ElementCore> Node<T> {
    pub fn get_grid(&mut self) -> Grid {
        self.grid.unwrap()
    }

    pub fn collides(&mut self, point: cgmath::Point2<f32>) -> bool {
        if let Some(el) = self.body.has_collider() {
            return el.contains(point);
        } else {
            return false;
        }
    }
}

//Node tree
#[derive(Clone)]
pub struct NodeWalker<T: ElementCore> {
    pub tree: Vec<Node<dyn ElementCore>>,
}

impl<T: ElementCore> NodeWalker<T> {
    pub fn create() -> NodeWalker<T> {
        NodeWalker { tree: Vec::new() }
    }

    pub fn add(&mut self, node: Node<T>) {
        if node.parent.is_some() {
            let mut node_clone = node.clone();
            let mut parent = self.find(node.parent.unwrap());
            let result = parent.get_grid().get_position(&node.area.clone().unwrap());
            node_clone.body.x(parent.body.get_x() + result.x);

            node_clone.body.y(parent.body.get_y() + result.y);

            if let Some(el) = node_clone.body.is_resizable() {
                el.width(result.width);
                el.height(result.height);
            }

            node_clone.body.build();
            self.tree.push(node_clone);
        } else {
            self.tree.push(node);
        }
    }

    pub fn find(&mut self, id: String) -> Node<T> {
        self.tree
            .iter()
            .find(|node| node.id == id)
            .expect("No Node found")
            .clone()
    }

    pub fn get_batch(&mut self) -> Batch {
        let mut batch = Batch::new();

        for node in self.tree.iter_mut() {
            batch.add_mesh(&node.body.mesh())
        }

        batch
    }
}
