use felin::definitions::{Mesh, Node};
use felin::mesh::{Image, Rectangle};
use felin::utils::{Batch, Grid, NodeWalker};
use felin::Event;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Element {
    nodes: NodeWalker,
    pub container: Batch,
}

impl Element {
    pub fn new() -> Self {
        let mut tree = NodeWalker::create();
        let container_rect = Rectangle {
            x: (800 / 2) as f32,
            y: 200.0,
            color: [0.52, 0.73, 0.94, 1.0],
            width: 800.0,
            height: 800.0,
            ..Default::default()
        };
        let container = tree.add(Node {
            grid: Some(Grid::new(
                container_rect.width,
                container_rect.height,
                "12/12",
            )),
            body: Rc::new(container_rect),
            parent: None,
            area: None,
            id: "container".to_string(),
        });
        tree.add(Node {
            grid: None,
            body: Rc::new(Rectangle {
                color: [0.52, 0.73, 0.94, 1.0],
                ..Default::default()
            }),
            parent: Some(container.clone()),
            area: Some("0/1/5/6".to_string()),
            id: "button_left".to_string(),
        });
        tree.add(Node {
            grid: None,
            body: Rc::new(Rectangle {
                color: [0.52, 0.73, 0.94, 1.0],
                ..Default::default()
            }),
            parent: Some(container.clone()),
            area: Some("11/12/5/6".to_string()),
            id: "button_right".to_string(),
        });
        tree.add(Node {
            grid: None,
            body: Rc::new(Rectangle {
                color: [0.52, 0.73, 0.94, 1.0],
                ..Default::default()
            }),
            parent: Some(container.clone()),
            area: Some("1/11/0/12".to_string()),
            id: "gallery".to_string(),
        });
        Self {
            container: tree.get_batch(),
            nodes: tree,
        }
    }
}
