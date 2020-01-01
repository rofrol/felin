use crate::definitions::{Mesh, Vertex};
use crate::utils::font::{FontBitmap, FontPallet, UvPosition};
use crate::utils::Batch;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub width: f32,
    pub height: f32,
    row_height: f32,
    last_char_position: cgmath::Vector2<f32>,

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
            last_char_position: cgmath::Vector2::new(0.0, 0.0),
            row_height: 10.0,
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

    pub fn build(&mut self, font: &FontPallet) -> Self {
        let mut batch = Batch::new();
        self.last_char_position = cgmath::Vector2::new(self.x, self.y);

        for key in self.text.clone().chars() {
            let character = font.get(key);
            let uv_positions = character.get_uv_position();

            //Push letter to new row
            if (self.last_char_position.x - self.x) > self.width {
                self.last_char_position =
                    cgmath::Vector2::new(self.x, self.last_char_position.y + 40.0);
            }

            let letter = self.create_letter(uv_positions, character);
            batch.add_mesh(&letter);
        }

        Self {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            last_char_position: self.last_char_position,
            row_height: self.row_height,
            color: self.color,
            vertices: batch.vertices,
            indices: batch.indices,
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
}
