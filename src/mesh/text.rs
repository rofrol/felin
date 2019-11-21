use crate::definitions::{Mesh, Vertex};
use cgmath::{self, prelude::*};


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub max_width: f32,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

#[allow(dead_code)]
impl Text {
    pub fn new() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            max_width: 100.0,
            vertices: Vec::new(),
            indices: Vec::new(),
            text: "".to_string()
        }
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }

    pub fn max_width(&mut self, width: f32) -> &mut Self {
        self.max_width = width;
        self
    }

    pub fn build(&self, font: &mut fontdue::Font) -> Self {

        let (metrics, bitmap) = font.rasterize('S', 17.0);

        Self {
            x: 100.0,
            y: 100.0,
            max_width: 100.0,
            vertices: Vec::new(),
            indices: Vec::new(),
            text: self.text.clone()
        }
    }

    pub fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }
}
