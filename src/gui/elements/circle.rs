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

struct VertCtr {
    color: [f32; 3],
}

impl VertexConstructor<tessellation::FillVertex, Vertex> for VertCtr {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> Vertex {
        Vertex { in_position: vertex.position.to_array(), in_color: self.color  }
    }
}


impl Circle {
    pub fn render(&self) -> RenderResult {

        let mut mesh: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);

        fill_circle(
            point(0.2, 1.5),
            1.0,
            &fill_options,
            &mut BuffersBuilder::new(&mut mesh, VertCtr {color: [1.0, 1.0, 1.0]}),
        ).unwrap();

        RenderResult {
            vertices: mesh.vertices,
            indices: mesh.indices,
        }
    }
}