use crate::definitions::{RenderResult, Vertex, RenderPass, Element};
use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;
use collision::primitive::ConvexPolygon;
use cgmath::{Point2};

#[derive(Debug)]
pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: [f32; 4],
    vertices: Vec<Vertex>,
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            radius: 50.0,
            x: 100.0,
            y: 100.0,
            color: [1.0, 1.0, 1.0, 1.0],
            vertices: Vec::new(),
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

    pub fn get_collider(&self) -> ConvexPolygon<f32> {
        let collider_verts: Vec<_> = self.vertices.clone().into_iter().map( |vert| Point2::new(vert.x(), vert.y())).collect();
        ConvexPolygon::new(collider_verts)
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
            vertices:self.vertices.clone(),
        }
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

        self.vertices = mesh.vertices.clone();
        
        rpass.draw_indexed(mesh.vertices, mesh.indices);
    }
}
