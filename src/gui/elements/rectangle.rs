use crate::gui::definitions::{Vertex, Element};
use wgpu;

pub struct Triangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Triangle {
    pub fn new() -> Self {
        Triangle {
            x: 10.0,
            y: 50.0,
            width: 100.0,
            height: 50.0,
        }
    }
}

impl Element for Triangle { 
    fn render(&self) -> Vec<Vertex> {

        let vertex_data = vec!(
                Vertex::new([-0.50, 0.0], [1.0, 1.0, 1.0]),
                Vertex::new([0.0, -0.50], [1.0, 1.0, 1.0]),
                Vertex::new([0.0, -0.20], [1.0, 1.0, 1.0]),
        );

        vertex_data
    }
}