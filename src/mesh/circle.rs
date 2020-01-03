use crate::definitions::{Mesh, Vertex};

use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;

use cgmath::{self, prelude::*};
use collision::{prelude::*, primitive, Aabb2};

use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub collider: Aabb2<f32>,
    pub color: [f32; 4],
    pub buffers: VertexBuffers<Vertex, u16>,
}

#[allow(dead_code)]
impl ElementCore for Circle {
    fn x(&mut self, x: f32) {
        self.x = x;
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

    fn y(&mut self, y: f32) {
        self.y = y;
    }

    fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.buffers.vertices.clone(),
            indices: self.buffers.indices.clone(),
        }
    }

    fn build(&mut self) {
        let mut mesh: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);
        //Draw vertices with Lyon
        fill_circle(
            point(self.x, self.y),
            self.radius,
            &fill_options,
            &mut BuffersBuilder::new(&mut mesh, |vertex: tessellation::FillVertex| Vertex {
                in_position: vertex.position.to_array(),
                in_color: self.color,
                tex_pos: [0.0, 0.0],
                texture_id: -1,
            }),
        )
        .unwrap();

        self.collider = self.get_collider();
        self.buffers = mesh;
    }

    fn is_resizable(&mut self) -> Option<&mut dyn ElememtResizable> {
        Some(self)
    }

    fn as_rc(&mut self) -> Rc<RefCell<dyn ElementCore>> {
        Rc::new(RefCell::new(self.clone()))
    }
}

impl ElememtResizable for Circle {
    fn radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    fn width(&mut self, width: f32) {}

    fn height(&mut self, height: f32) {}
}

impl ElementCollider for Circle {
    fn contains(&self, point: cgmath::Point2<f32>) -> bool {
        self.collider.contains(&point)
    }

    fn get_collider(&self) -> Aabb2<f32> {
        let transform: cgmath::Decomposed<cgmath::Vector2<f32>, cgmath::Basis2<f32>> =
            cgmath::Decomposed {
                scale: 1.0,
                rot: Rotation2::from_angle(cgmath::Rad(0.0)),
                disp: cgmath::Vector2::new(self.x, self.y),
            };
        return primitive::Circle::new(self.radius)
            .compute_bound()
            .transform(&transform);
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 50.0,
            x: 100.0,
            y: 100.0,
            collider: Aabb2 {
                min: cgmath::Point2::new(0.0, 0.0),
                max: cgmath::Point2::new(0.0, 0.0),
            },
            color: [1.0, 1.0, 1.0, 1.0],
            buffers: VertexBuffers::new(),
        }
    }
}
