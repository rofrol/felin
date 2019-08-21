use crate::gui::definitions::{Vertex, Element};
use wgpu;

pub struct Rectangle {
    pub name: String,
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            name: String::from("rectangle"),
        }
    }
}

impl Element for Rectangle { 
    fn render(&self) -> Vec<Vertex> {

        let vertex_data = vec!(
                Vertex::new([-0.50, 0.0], [1.0, 1.0, 1.0]),
                Vertex::new([0.0, -0.50], [1.0, 1.0, 1.0]),
                Vertex::new([0.0, -0.20], [1.0, 1.0, 1.0]),
                Vertex::new([0.50, 0.0], [1.0, 1.0, 1.0]),
                Vertex::new([0.50, 0.8], [1.0, 1.0, 1.0]),
                Vertex::new([0.0, 0.20], [1.0, 1.0, 1.0]),
        );

        vertex_data
    }
}