use crate::definitions::{Mesh, Vertex};
use crate::utils::Batch;
use crate::utils::{FontBitmap, FontPallet};

use cgmath::{self, prelude::*};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub width: f32,
    pub height: f32,
    texture_index: i32,
    color: [f32; 4],
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

#[allow(dead_code)]
impl Text {
    pub fn new() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            width: 100.0,
            height: 100.0,
            color: [1.0, 1.0, 1.0, 1.0],
            texture_index: 0,
            vertices: Vec::new(),
            indices: Vec::new(),
            text: "".to_string(),
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

    fn create_letter(&mut self, letter: &FontBitmap) -> Mesh {
        let vertices = vec![
            //Left top corner
            Vertex::new([self.x, self.y], self.color, [0.0, 0.0], self.texture_index),
            //Right top corner
            Vertex::new(
                [self.x + self.width, self.y],
                self.color,
                [1.0, 0.0],
                self.texture_index,
            ),
            //Right bottom corner
            Vertex::new(
                [self.x + self.width, self.y + self.height],
                self.color,
                [1.0, 1.0],
                self.texture_index,
            ),
            //Left bottom
            Vertex::new(
                [self.x, self.y + self.height],
                self.color,
                [0.0, 1.0],
                self.texture_index,
            ),
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh {
            vertices,
            indices
        }
    }

    pub fn build(&mut self, font: &FontPallet) -> Self {
        let text = "t";
        let batch = Batch::new();

        for key in text.chars() {
            let character_mesh = self.create_letter(font.get(key));
        }

        Self {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: self.color,
            vertices: Vec::new(),
            indices: Vec::new(),
            texture_index: self.texture_index,
            text: self.text.clone(),
        }
    }

    pub fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }
}
