use crate::definitions::{Mesh, Vertex};
use crate::prelude::*;
use collision::{prelude::*, primitive, Aabb2};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Image {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub collider: Aabb2<f32>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub color: [f32; 4],
    pub texture_index: i32,
}

impl Default for Image {
    fn default() -> Self {
        Self {
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
}

#[allow(dead_code)]
impl ElementCore for Image {
    fn color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    fn x(&mut self, x: f32) {
        self.x = x;
    }

    fn y(&mut self, y: f32) {
        self.y = y;
    }

    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }

    fn build(&mut self) {
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

        self.indices = vec![0, 1, 2, 2, 3, 0];
        self.vertices = vertices;
        self.collider = self.get_collider();
    }

    fn is_resizable(&mut self) -> Option<&mut dyn ElememtResizable> {
        Some(self)
    }

    fn as_rc(&mut self) -> Rc<RefCell<dyn ElementCore>> {
        Rc::new(RefCell::new(self.clone()))
    }
}

impl ElementCollider for Image {
    fn get_collider(&self) -> Aabb2<f32> {
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

    fn contains(&self, point: cgmath::Point2<f32>) -> bool {
        self.collider.contains(&point)
    }
}

impl ElememtResizable for Image {
    fn width(&mut self, width: f32) {
        self.width = width;
    }

    fn radius(&mut self, radius: f32) {}

    fn height(&mut self, height: f32) {
        self.height = height;
    }
}

impl ElementImage for Image {
    fn use_texture(&mut self, index: i32) {
        self.texture_index = index;
    }
}
