use crate::definitions::{Mesh, Vertex};

use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;

use collision::{prelude::*, primitive, Aabb2};

#[derive(Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
    pub collider: Aabb2<f32>,
    buffers: VertexBuffers<Vertex, u16>,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle {
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

    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }

    pub fn build(&mut self) -> Rectangle {
        let mut buffers: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);

        //Draw vertices with Lyon
        fill_rectangle(
            &rect(self.x, self.y, self.width, self.height),
            &fill_options,
            &mut BuffersBuilder::new(&mut buffers, |vertex: tessellation::FillVertex| {
                Vertex::new(vertex.position.to_array(), self.color, [0.0, 0.0], -1)
            }),
        )
        .unwrap();

        self.buffers = buffers.clone();

        Rectangle {
            x: self.x,
            y: self.y,
            buffers: buffers,
            height: self.height,
            collider: self.get_collider(),
            width: self.width,
            color: self.color,
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
