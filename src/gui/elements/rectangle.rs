use crate::gui::definitions::{ Vertex};
use cgmath;

pub struct Rectangle {
    width: Option<u32>,
    height: Option<u32>,
    vertices: Option<Vec<Vertex>>,
    transform: Option<cgmath::Matrix4<f32>>,
}

impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle {
            width:None,
            height:None,
            vertices:None,
            transform:None,
        }
    }

    pub fn width(&mut self, width: u32) {
        self.width = Some(width);
    }

    pub fn height(&mut self, height: u32) {
        self.height = Some(height);
    }
}
