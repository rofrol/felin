use crate::definitions::{Mesh, Vertex};
use crate::utils::font::{FontBitmap, FontPallet, UvPosition};
use crate::utils::Batch;
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

    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: f32) -> &mut Self {
        self.height = height;
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

    fn create_letter(&mut self, letter: UvPosition) -> Mesh {
        println!("{:?}", letter);
        let vertices = vec![
            //Left top corner
            Vertex::new(
                [self.x, self.y],
                self.color,
                [letter.x[0], letter.y[0]],
                self.texture_index,
            ),
            //Right top corner
            Vertex::new(
                [self.x + self.width, self.y],
                self.color,
                [letter.x[1], letter.y[0]],
                self.texture_index,
            ),
            //Right bottom corner
            Vertex::new(
                [self.x + self.width, self.y + self.height],
                self.color,
                [letter.x[1], letter.y[1]],
                self.texture_index,
            ),
            //Left bottom
            Vertex::new(
                [self.x, self.y + self.height],
                self.color,
                [letter.x[0], letter.y[1]],
                self.texture_index,
            ),
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh { vertices, indices }
    }

    pub fn build(&mut self, font: &FontPallet) -> Self {
        let text = "t";
        let mut batch = Batch::new();

        for key in text.chars() {
            let uv_positions = font.get(key).get_uv_position();
            let letter = self.create_letter(uv_positions);
            batch.add_mesh(&letter);
        }
        self.vertices = batch.vertices;
        self.indices = batch.indices;
        Self {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: self.color,
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
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
