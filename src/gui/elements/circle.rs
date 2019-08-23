use lyon::math::*;
use lyon::tessellation::geometry_builder::{VertexConstructor, VertexBuffers, BuffersBuilder};
use lyon::tessellation::{StrokeOptions, FillOptions};
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation;

use crate::definitions::{Vertex, Element, RenderResult};


pub struct Circle {
    width: f32,
    height: f32,
    color: [f32; 3], 
}


impl Circle {
    pub fn new() -> Circle {
        Circle {
            width: 100.0,
            height: 100.0,
            color: [1.0, 1.0, 1.0],
        }
    }
}

impl Circle {
    pub fn render(&self) -> RenderResult {

        let mut mesh: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);

        fill_circle(
            point(0.0, 0.0),
            200.0,
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