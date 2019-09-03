use crate::definitions::{Element, RenderPass, RenderResult, Vertex};
use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;

use collision::{prelude::*, primitive, Ray2};

use cgmath::{self, prelude::*, Point2, Rad, Rotation2, Vector2};

#[derive(Debug)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: [f32; 4],
    pub collider: Option<primitive::Circle<f32>>,
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            radius: 50.0,
            x: 100.0,
            y: 100.0,
            color: [1.0, 1.0, 1.0, 1.0],
            collider: None,
        }
    }


    pub fn x(&mut self, x: f32) -> &mut Circle {
        self.x = x;
        self
    }

    pub fn radius(&mut self, radius: f32) -> &mut Circle {
        self.radius = radius;
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Circle {
        self.y = y;
        self
    }

    pub fn init(&mut self) -> &mut Self {
        self
    }

    pub fn build(&self) -> Circle {
        Circle {
            radius: self.radius,
            x: self.x,
            y: self.y,
            color: self.color,
            collider: None,
        }
    }

    pub fn collides(&self, point: Point2<f32>) -> bool {
        let mut collider_result: bool = false;
        if let Some(collider) = &self.collider {
            let transform: cgmath::Decomposed<Vector2<f32>, cgmath::Basis2<f32>> =
                cgmath::Decomposed {
                    scale: 1.0,
                    rot: Rotation2::from_angle(Rad(0.0)),
                    disp: Vector2::new(self.x, self.y),
                };
            if collider.compute_bound().transform(&transform).contains(&point) {
                collider_result = true
            }
        }
        collider_result
    }
}

impl Element for Circle {
    fn render(&mut self, rpass: &mut RenderPass) {
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

        self.collider = Some(primitive::Circle::new(self.radius));
        rpass.draw_indexed(mesh.vertices, mesh.indices);
    }
}
