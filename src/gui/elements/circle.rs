use crate::definitions::{RenderResult, Vertex, RenderPass, Element};
use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;

use collision::{
    prelude::*,
    primitive,
    Ray2,
};

use cgmath::{self, Point2, Vector2};
use cgmath::prelude::*;

#[derive(Debug)]
pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: [f32; 4],
    collider: Option<primitive::Circle<f32>>,
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
            x:self.x,
            y:self.y,
            color:self.color,
            collider: None,
        }
    }

    pub fn on_click(&self, point: Point2<f32>) -> bool {
        if let Some(collider) = &self.collider {
            // let ray = Ray2::new(point, Vector2::new(0., 0.));
            // let transform: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
            // println!("{:?}", collider.intersection_transformed(&ray, &transform));
        }

        false
    }
}

impl Element for Circle {
    fn render(&mut self, rpass: &mut RenderPass) {
        let mut mesh: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);
        
        fill_circle(
            point(self.x, self.y),
            self.radius,
            &fill_options,
            &mut BuffersBuilder::new(&mut mesh, |vertex: tessellation::FillVertex| { 
                Vertex {
                    in_position: vertex.position.to_array(),
                    in_color: self.color,
                }
            }),
        )
        .unwrap();

        self.collider = Some(primitive::Circle::new(self.radius));
        
        rpass.draw_indexed(mesh.vertices, mesh.indices);
    }
}
