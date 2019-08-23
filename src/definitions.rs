use cgmath::{Point2, Vector2, Matrix4};
use wgpu;

pub use crate::engine::{RenderPass};

pub use crate::gui::{ElementRegistry};

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

///////////////////////////////////////////////////////////////////////////
// Vertex
///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub in_position: [f32; 2],
    pub in_color: [f32; 3],
}

impl Vertex {
    pub fn new(in_position: [f32; 2], in_color: [f32; 3]) -> Self {
        Self {in_position, in_color}
    }
}


///////////////////////////////////////////////////////////////////////////
// Elements
///////////////////////////////////////////////////////////////////////////

pub trait Element {
    fn render(&self, rpass: &mut RenderPass);
}

pub struct Widget {
    pub id: String,
    pub body: Box<dyn Element>,
}


///////////////////////////////////////////////////////////////////////////
// Rendering
///////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct RenderResult {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}


