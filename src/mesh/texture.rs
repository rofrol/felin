use crate::definitions::{Mesh, Vertex};
use collision::{prelude::*, primitive, Aabb2};

#[derive(Clone)]
pub struct Image {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub collider: Aabb2<f32>,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    pub color: [f32; 4],
    texture_index: i32,
}

#[allow(dead_code)]
impl Image {
    pub fn new() -> Image {
        Image {
            x: 100.0,
            y: 100.0,
            width: 100.0,
            height: 100.0,
            color: [0.0, 0.0, 0.0, 0.0],
            collider: Aabb2 {
                min: cgmath::Point2::new(0.0, 0.0),
                max: cgmath::Point2::new(0.0, 0.0),
            },
            vertices: Vec::new(),
            indices: Vec::new(),
            texture_index: 0,
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

    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn use_texture(&mut self, index: i32) -> &mut Self {
        self.texture_index = index;
        self
    }

    pub fn build(&mut self) -> Image {
        let vertices = vec![
            //Left top corner
            Vertex {
                in_position: [self.x, self.y],
                in_color: self.color,
                tex_pos: [0.0, 0.0],
                texture_id: self.texture_index,
            },
            //Right top corner
            Vertex {
                in_position: [self.x + self.width, self.y],
                in_color: self.color,
                tex_pos: [1.0, 0.0],
                texture_id: self.texture_index,
            },
            //Right bottom corner
            Vertex {
                in_position: [self.x + self.width, self.y + self.height],
                in_color: self.color,
                tex_pos: [1.0, 1.0],
                texture_id: self.texture_index,
            },
            //Left bottom
            Vertex {
                in_position: [self.x, self.y + self.height],
                in_color: self.color,
                tex_pos: [0.0, 1.0],
                texture_id: self.texture_index,
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Image {
            x: self.x,
            y: self.y,
            height: self.height,
            vertices,
            indices,
            collider: self.get_collider(),
            color: self.color,
            width: self.width,
            texture_index: self.texture_index,
        }
    }

    pub fn get_collider(&self) -> Aabb2<f32> {
        let transform: cgmath::Decomposed<cgmath::Vector2<f32>, cgmath::Basis2<f32>> =
            cgmath::Decomposed {
                scale: 1.0,
                rot: cgmath::Rotation2::from_angle(cgmath::Rad(0.0)),
                disp: cgmath::Vector2::new(
                    self.x + (self.width / 2.0),
                    self.y + (self.height / 2.0),
                ),
            };
        primitive::Rectangle::new(self.width, self.height)
            .compute_bound()
            .transform(&transform)
    }

    pub fn contains(&self, point: cgmath::Point2<f32>) -> bool {
        self.collider.contains(&point)
    }

    pub fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }
}
