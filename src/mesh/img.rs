use crate::definitions::{Mesh, Vertex};
use crate::prelude::*;
use crate::utils::Style;
use collision::{prelude::*, primitive, Aabb2};

#[derive(Clone)]
pub struct Image {
    pub style: Style,
    pub collider: Aabb2<f32>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub color: [f32; 4],
    pub texture: i32,
    pub id: Option<String>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            style: Style::default(),
            color: [0.0, 0.0, 0.0, 0.0],
            collider: Aabb2 {
                min: cgmath::Point2::new(0.0, 0.0),
                max: cgmath::Point2::new(0.0, 0.0),
            },
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: 0,
            id: None,
        }
    }
}

#[allow(dead_code)]
impl ElementCore for Image {
    type Vertex = Vertex;
    fn build(&mut self) {
        let vertices = vec![
            //Left top corner
            Vertex {
                in_position: [self.style.x, self.style.y],
                in_color: self.color,
                tex_pos: [0.0, 0.0],
                texture_id: self.texture,
            },
            //Right top corner
            Vertex {
                in_position: [self.style.x + self.style.width, self.style.y],
                in_color: self.color,
                tex_pos: [1.0, 0.0],
                texture_id: self.texture,
            },
            //Right bottom corner
            Vertex {
                in_position: [
                    self.style.x + self.style.width,
                    self.style.y + self.style.height,
                ],
                in_color: self.color,
                tex_pos: [1.0, 1.0],
                texture_id: self.texture,
            },
            //Left bottom
            Vertex {
                in_position: [self.style.x, self.style.y + self.style.height],
                in_color: self.color,
                tex_pos: [0.0, 1.0],
                texture_id: self.texture,
            },
        ];

        self.indices = vec![0, 1, 2, 2, 3, 0];
        self.vertices = vertices;
        self.collider = self.get_collider();
    }

    fn get_style(&self) -> Style {
        self.style
    }

    fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn mesh(&self) -> Mesh<Vertex> {
        Mesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }
}

impl ElementCollider for Image {
    fn get_collider(&self) -> Aabb2<f32> {
        let transform: cgmath::Decomposed<cgmath::Vector2<f32>, cgmath::Basis2<f32>> =
            cgmath::Decomposed {
                scale: 1.0,
                rot: cgmath::Rotation2::from_angle(cgmath::Rad(0.0)),
                disp: cgmath::Vector2::new(
                    self.style.x + (self.style.width / 2.0),
                    self.style.y + (self.style.height / 2.0),
                ),
            };
        primitive::Rectangle::new(self.style.width, self.style.height)
            .compute_bound()
            .transform(&transform)
    }

    fn contains(&self, point: cgmath::Point2<f32>) -> bool {
        self.collider.contains(&point)
    }
}
