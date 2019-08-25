pub mod elements;

use crate::definitions::{Node, Element};
use froggy;

pub struct Widget {
    pub entries: froggy::Storage<Node>,
}

impl Widget {
    pub fn new(id: &str, element: Box<dyn Element>) -> Widget {
        let node_element: Node = Node {
            id: String::from(id),
            body: element,
            parent_node: None,
            children: Vec::new(),
        };

        let mut widget: Widget = Widget {
             entries: froggy::Storage::new(),
         };

        let node_pointer: froggy::Pointer<Node> = widget.entries.create(node_element);

        widget
    }


    pub fn append(&mut self, id: &str, parentId: &str, element: Box<dyn Element>) -> &mut Self {
        let parent_node:froggy::Pointer<Node> = self.get_node(&parentId);

        let node_element:Node = Node {
            id: String::from(id),
            body: element,
            parent_node: Some(parent_node.clone()),
            children: Vec::new(),
        };

        let node_pointer: froggy::Pointer<Node> = self.entries.create(node_element);

        self.entries[&parent_node].children.push(node_pointer.clone());

        self
    }

    fn get_node(&mut self, id: &str) -> froggy::Pointer<Node> {
        let item = self.entries.iter_all().find(|node| node.id == String::from(id)).unwrap();
        self.entries.pin(&item)
    }


    pub fn find(&mut self, id: &str) -> &Node {
        let item = self.entries.iter_all().find(|node| node.id == String::from(id)).unwrap();
        let node = &self.entries[&self.entries.pin(&item)];

        node
    }
}