use super::batch::Batch;
use crate::definitions::Mesh;
pub use crate::definitions::Node;
use crate::prelude::*;
use crate::utils::Grid;
use std::rc::Rc;

impl Node {
    pub fn get_grid(&mut self) -> Grid {
        self.grid.unwrap()
    }

    pub fn mesh(&mut self) -> Mesh {
        Rc::get_mut(&mut self.body).unwrap().mesh()
    }
}

//Node tree
pub struct NodeWalker {
    tree: froggy::Storage<Node>,
}

impl NodeWalker {
    pub fn create() -> NodeWalker {
        NodeWalker {
            tree: froggy::Storage::new(),
        }
    }

    pub fn get_tree(&mut self) -> &mut froggy::Storage<Node> {
        &mut self.tree
    }

    pub fn cursor(&mut self) -> froggy::Cursor<Node> {
        self.tree.cursor()
    }

    pub fn add(&mut self, node: &mut Node) -> froggy::Pointer<Node> {
        if node.parent.is_some() {
            let parent = &mut self.tree[&node.parent.clone().unwrap()];
            let result = parent.get_grid().get_position(&node.area.clone().unwrap());
            let parent_body = Rc::get_mut(&mut parent.body).expect("failed to get rc 1");

            let element = Rc::get_mut(&mut node.body).expect("failed to get rc 2");

            element.x(parent_body.get_x() + result.x);
            element.y(parent_body.get_y() + result.y);

            if let Some(el) = element.is_resizable() {
                el.width(result.width);
                el.height(result.height);
            }
            
            element.build();

            return self.tree.create(node.clone());
        } else {
            return self.tree.create(node.clone());
        }
    }

    pub fn get(&mut self, pointer: &froggy::Pointer<Node>) -> Node {
        self.tree[pointer].clone()
    }

    pub fn find(&self, id: &str) -> Node {
        let index = self
            .tree
            .iter_all()
            .find(|x| x.id == id.to_string())
            .unwrap();
        self.tree[&self.tree.pin(&index)].clone()
    }

    pub fn get_batch(&mut self) -> Batch {
        let mut batch = Batch::new();

        for node in self.tree.iter_mut() {
            let element = Rc::get_mut(&mut node.body).unwrap();
            batch.add_mesh(&element.mesh())
        }

        batch
    }
}
