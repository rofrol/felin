use crate::gui::definitions::{Element, Vertex};
use cgmath;

pub struct Square {
    width: Option<u32>,
    height: Option<u32>,
    vertices: Option<Vec<Vertex>>,
    transform: Option<cgmath::Matrix4<f32>>,
}

impl Element for Square {
    fn render(&self) -> Vec<Vertex> {
    //    return match self.vertices {
    //         Some(ref vert) => return vert),
    //         None => Vertex::new([0.0, 0.0], [1.0, 1.0, 1.0]),
    //     }
        [Vertex::new([0.0, 0.0], [1.0, 1.0, 1.0])].to_vec()
    }
}