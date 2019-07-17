use lyon::math::*;
use lyon::tessellation::geometry_builder::{VertexConstructor, VertexBuffers, BuffersBuilder};
use lyon::tessellation::{FillOptions, FillVertex};
use lyon::tessellation::basic_shapes::*;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
}


// A very simple vertex constructor that only outputs the vertex position
struct VertexCtor;
impl VertexConstructor<FillVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex { position: vertex.position.to_array(), }
    }
}


pub struct Palette {
   pub mesh: VertexBuffers<Vertex, u16>,
   fill_options: FillOptions,
}


impl Palette {
    pub fn new() -> Palette {

        Palette {
            mesh: VertexBuffers::new(),
            fill_options: FillOptions::tolerance(0.01),
        }
    }

    pub fn circle(&mut self) {
        fill_circle(
            point(200.0, 100.0),
            2.0,
            &self.fill_options,
            &mut BuffersBuilder::new(&mut self.mesh, VertexCtor),
        ).unwrap();
    }


    pub fn rectangle(&mut self) {
        fill_rectangle(
            &rect(150.0, 250.0, 250.0, 150.0),
            &self.fill_options,
            &mut BuffersBuilder::new(&mut self.mesh, VertexCtor),
        ).unwrap();
    }
}