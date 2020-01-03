use crate::definitions::{Mesh, Vertex};

use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;

use collision::{prelude::*, primitive, Aabb2};

use crate::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
    pub collider: Aabb2<f32>,
    pub buffers: VertexBuffers<Vertex, u16>,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            x: 50.0,
            y: 50.0,
            width: 100.0,
            height: 100.0,
            collider: Aabb2 {
                min: cgmath::Point2::new(0.0, 0.0),
                max: cgmath::Point2::new(0.0, 0.0),
            },
            buffers: VertexBuffers::new(),
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

#[allow(dead_code)]
impl ElementCore for Rectangle {
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

    fn color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.buffers.vertices.clone(),
            indices: self.buffers.indices.clone(),
        }
    }

    fn build(&mut self) {
        let mut buffers: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);

        //Draw vertices with Lyon
        fill_rectangle(
            &rect(self.x, self.y, self.width, self.height),
            &fill_options,
            &mut BuffersBuilder::new(&mut buffers, |vertex: tessellation::FillVertex| Vertex {
                in_position: vertex.position.to_array(),
                in_color: self.color,
                tex_pos: [0.0, 0.0],
                texture_id: -1,
            }),
        )
        .unwrap();

        self.buffers = buffers;
        self.collider = self.get_collider();
    }

    fn is_resizable(&mut self) -> Option<&mut dyn ElememtResizable> { Some(self) }

    fn as_rc(&mut self) -> Rc<RefCell<dyn ElementCore>> {
        Rc::new(RefCell::new(self.clone()))
    }
}

impl ElememtResizable for Rectangle {
    fn width(&mut self, width: f32) {
        self.width = width;
    }

    fn height(&mut self, height: f32) {
        self.height = height;
    }

    fn radius(&mut self, radius: f32) {}
}

impl ElementCollider for Rectangle {
    fn contains(&self, point: cgmath::Point2<f32>) -> bool {
        self.collider.contains(&point)
    }

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
}
