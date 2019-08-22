use lyon::math::*;
use lyon::tessellation::geometry_builder::{VertexConstructor, VertexBuffers, BuffersBuilder};
use lyon::tessellation::{StrokeOptions, FillOptions};
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation;

use crate::definitions::{Vertex, Element, RenderResult};


struct VertexCtor;

impl VertexConstructor<tessellation::FillVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> Vertex {
        Vertex { in_position: vertex.position.to_array(), in_color: [1.0, 1.0, 1.0] }
    }
}


struct Circle {
    width: f32,
    height: f32,
}


impl Circle {
    pub fn new() -> Circle {
        Circle {
            width: 100.0,
            height: 100.0,
        }
    }
}


impl Circle {
    pub fn render(&self) -> RenderResult {

        let mut mesh: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);

        fill_circle(
            point(22.0, 3.0),
            2.0,
            &fill_options,
            &mut BuffersBuilder::new(&mut mesh, VertexCtor),
        ).unwrap();

        RenderResult {
            vertices: mesh.vertices,
            indices: mesh.indices.
        }
    }
}