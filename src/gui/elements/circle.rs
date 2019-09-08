use crate::definitions::{Element, Mesh, Vertex};

use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;

use cgmath::{self, prelude::*, Point2, Rad, Rotation2, Vector2};
use collision::{prelude::*, primitive, Aabb2};

#[derive(Debug)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: [f32; 4],
    collider: Aabb2<f32>,
    transform: cgmath::Matrix4<f32>,
    update: bool,
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            radius: 50.0,
            x: 100.0,
            y: 100.0,
            color: [1.0, 1.0, 1.0, 1.0],
            collider: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            transform: cgmath::Matrix4::identity(),
            update: true,
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

    pub fn y(&mut self, y: f32) -> &mut Self{
        self.y = y;
        self
    }

    pub fn build(&self) -> Circle {
        Circle {
            radius: self.radius,
            x: self.x,
            y: self.y,
            color: self.color,
            collider: self.collider,
            transform: self.transform,
            update: self.update,
        }
    }

    pub fn get_collider(&self) -> Aabb2<f32> {
        let transform: cgmath::Decomposed<Vector2<f32>, cgmath::Basis2<f32>> = cgmath::Decomposed {
            scale: 1.0,
            rot: Rotation2::from_angle(Rad(0.0)),
            disp: Vector2::new(self.x, self.y),
        };
        primitive::Circle::new(self.radius)
            .compute_bound()
            .transform(&transform)
    }

    pub fn contains(&self, point: Point2<f32>) -> bool {
        self.collider.contains(&point)
    }
}

impl Element for Circle {
    fn render(&mut self) -> Mesh {
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
            }),
        )
        .unwrap();

        self.collider = self.get_collider();

        Mesh {
            indices: mesh.indices,
            vertices: mesh.vertices,
            transform: self.transform,
            update: self.update,
        }
    }
}
