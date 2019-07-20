use cgmath::{Point2, Vector2, Matrix4};
use wgpu;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


///////////////////////////////////////////////////////////////////////////
// Mesh
///////////////////////////////////////////////////////////////////////////


pub struct Mesh {
    pub key: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub rendered: bool,
}


///////////////////////////////////////////////////////////////////////////
// Vertex
///////////////////////////////////////////////////////////////////////////



#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    in_position: [f32; 2],
    in_color: [f32; 3],
}

impl Vertex {
    pub fn new(in_position: [f32; 2], in_color: [f32; 3]) -> Self {
        Self {in_position, in_color}
    }

    pub fn x(&self) -> f32 {
        self.in_position[0]
    }

    pub fn y(&self) -> f32 {
        self.in_position[1]
    }
}
