use felin::mesh::{Image, Rectangle};
use felin::prelude::*;
use felin::utils::{Batch, Grid, Node, NodeWalker};

#[allow(dead_code)]
pub struct Element {
    nodes: NodeWalker,
    pub container: Batch,
}

impl Element {
    pub fn new() -> Self {
        let mut tree = NodeWalker::create();
        let mut container_rect = Rectangle {
            x: (800) as f32,
            y: 200.0,
            color: [0.52, 0.73, 0.94, 1.0],
            width: 800.0,
            height: 800.0,
            ..Default::default()
        };

        tree.add(Node {
            grid: Some(Grid::new(
                container_rect.width,
                container_rect.height,
                "12/12",
            )),
            body: container_rect.as_rc(),
            parent: None,
            area: None,
            id: "container".to_string(),
        });

        tree.add(Node {
            grid: None,
            body: Rectangle {
                color: [0.52, 0.73, 0.94, 1.0],
                ..Default::default()
            }
            .as_rc(),
            parent: Some("container".to_string()),
            area: Some("0/1/5/6".to_string()),
            id: "button_left".to_string(),
        });

        tree.add(Node {
            grid: None,
            body: Rectangle {
                color: [0.52, 0.73, 0.94, 1.0],
                ..Default::default()
            }
            .as_rc(),
            parent: Some("container".to_string()),
            area: Some("11/12/5/6".to_string()),
            id: "button_right".to_string(),
        });

        tree.add(Node {
            grid: None,
            body: Rectangle {
                color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            }
            .as_rc(),
            parent: Some("container".to_string()),
            area: Some("1/11/1/12".to_string()),
            id: "gallery".to_string(),
        });

        Self {
            container: tree.get_batch(),
            nodes: tree,
        }
    }
}
