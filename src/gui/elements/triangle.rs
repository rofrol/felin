use crate::definitions::{Element, RenderPass, Vertex};

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
    fn render(&self, rpass: &mut RenderPass) {
        let vertex_data = vec![
            Vertex::new([-0.50, 0.0], [1.0, 1.0, 1.0, 1.0]),
            Vertex::new([0.0, -0.50], [1.0, 1.0, 1.0, 1.0]),
            Vertex::new([0.0, -0.20], [1.0, 1.0, 1.0, 1.0]),
        ];

        rpass.draw(vertex_data);
    }
}
