use crate::definitions::{Mesh, Vertex};
use crate::prelude::*;
use crate::utils::font::{FontBitmap, FontPallet, UvPosition};
use crate::utils::Batch;
use crate::utils::Style;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Text {
    pub style: Style,

    pub text: String,
    pub row_height: f32,
    pub last_char_position: cgmath::Vector2<f32>,

    pub texture_index: i32,
    pub color: [f32; 4],
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub scale: f32,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            style: Style::default(),
            last_char_position: cgmath::Vector2::new(0.0, 0.0),
            row_height: 10.0,
            color: [1.0, 1.0, 1.0, 1.0],
            texture_index: 0,
            vertices: Vec::new(),
            indices: Vec::new(),
            text: "".to_string(),
            scale: 1.0,
        }
    }
}

#[allow(dead_code)]
impl ElementCore for Text {
    fn build(&mut self) {}
    fn get_style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }
}

impl Text {
    pub fn color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }

    fn create_letter(&mut self, uv: UvPosition, character: &FontBitmap) -> Mesh {
        let vertices = vec![
            //Left top corner
            Vertex {
                in_position: [
                    self.last_char_position.x,
                    self.last_char_position.y + character.offset_y,
                ],
                in_color: self.color,
                tex_pos: [uv.x[0], uv.y[0]],
                texture_id: self.texture_index,
            },
            //Right top corner
            Vertex {
                in_position: [
                    self.last_char_position.x + character.width as f32,
                    self.last_char_position.y + character.offset_y,
                ],
                in_color: self.color,
                tex_pos: [uv.x[1], uv.y[0]],
                texture_id: self.texture_index,
            },
            //Right bottom corner
            Vertex {
                in_position: [
                    self.last_char_position.x + character.width as f32,
                    self.last_char_position.y + character.height as f32 + character.offset_y,
                ],
                in_color: self.color,
                tex_pos: [uv.x[1], uv.y[1]],
                texture_id: self.texture_index,
            },
            //Left bottom
            Vertex {
                in_position: [
                    self.last_char_position.x,
                    self.last_char_position.y + character.height as f32 + character.offset_y,
                ],
                in_color: self.color,
                tex_pos: [uv.x[0], uv.y[1]],
                texture_id: self.texture_index,
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        self.last_char_position = self.last_char_position
            + cgmath::Vector2 {
                x: character.width as f32,
                y: 0.0,
            };

        Mesh { vertices, indices }
    }

    pub fn build_text(&mut self, font: &FontPallet) {
        let mut batch = Batch::new();
        self.last_char_position = cgmath::Vector2::new(self.style.x, self.style.y);

        for key in self.text.clone().chars() {
            let character = font.get(key);
            let uv_positions = character.get_uv_position();

            //Push letter to new row
            if (self.last_char_position.x - self.style.x) > self.style.width {
                self.last_char_position =
                    cgmath::Vector2::new(self.style.x, self.last_char_position.y + 40.0);
            }

            let letter = self.create_letter(uv_positions, character);
            batch.add_mesh(&letter);
        }

        self.vertices = batch.vertices;
        self.indices = batch.indices;
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }
}

impl ElememtResizable for Text {
    fn width(&mut self, width: f32) {
        self.style.width = width;
    }

    fn height(&mut self, height: f32) {
        self.style.height = height;
    }

    fn radius(&mut self, _radius: f32) {}
}
