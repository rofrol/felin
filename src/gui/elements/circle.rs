use lyon::math::*;
use lyon::tessellation::geometry_builder::{VertexConstructor, VertexBuffers, BuffersBuilder};
use lyon::tessellation::{StrokeOptions, FillOptions};
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation;
use crate::definitions::{Vertex, Element, RenderResult};

pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: [f32; 3], 
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            radius: 50.0,
            x:100.0,
            y:100.0,
            color: [1.0, 1.0, 1.0],
        }
    }
}

impl Circle {
    pub fn render(&self) -> RenderResult {

        let mut mesh: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);

        fill_circle(
            point(self.x, self.y),
            self.radius,
            &fill_options,
            &mut BuffersBuilder::new(&mut mesh, |vertex : tessellation::FillVertex| {
                Vertex {
                    in_position: vertex.position.to_array(),
                    in_color: self.color,
                }
            }),
        ).unwrap();

        RenderResult {
            vertices: mesh.vertices,
            indices: mesh.indices,
        }
    }
}