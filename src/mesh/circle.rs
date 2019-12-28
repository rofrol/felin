use crate::definitions::{Mesh, Vertex};

use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;

use cgmath::{self, prelude::*};
use collision::{prelude::*, primitive, Aabb2};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub collider: Aabb2<f32>,
    pub color: [f32; 4],
    buffers: VertexBuffers<Vertex, u16>,
}

#[allow(dead_code)]
impl Circle {
    pub fn new() -> Circle {
        Circle {
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

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }

    pub fn radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }

    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }

    pub fn build(&self) -> Circle {
        let mut mesh: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);
        //Draw vertices with Lyon
        fill_circle(
            point(self.x, self.y),
            self.radius,
            &fill_options,
            &mut BuffersBuilder::new(&mut mesh, |vertex: tessellation::FillVertex| {
                Vertex::new(vertex.position.to_array(), self.color, [0.0, 0.0], -1)
            }),
        )
        .unwrap();

        Circle {
            radius: self.radius,
            collider: self.get_collider(),
            x: self.x,
            y: self.y,
            color: self.color,
            buffers: mesh,
        }
    }

    pub fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.buffers.vertices.clone(),
            indices: self.buffers.indices.clone(),
        }
    }

    pub fn contains(&self, point: cgmath::Point2<f32>) -> bool {
        self.collider.contains(&point)
    }

    pub fn get_collider(&self) -> Aabb2<f32> {
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
