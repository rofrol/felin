use cgmath::{self, prelude::*, Matrix4, Point2};
use froggy;

pub use crate::engine::Frame;
pub use crate::gui::Widget;

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
    pub in_color: [f32; 4],
}

impl Vertex {
    pub fn new(in_position: [f32; 2], in_color: [f32; 4]) -> Self {
        Self {
            in_position,
            in_color,
        }
    }
    pub fn x(&self) -> f32 {
        self.in_position[0]
    }

    pub fn y(&self) -> f32 {
        self.in_position[1]
    }
}

///////////////////////////////////////////////////////////////////////////
// Elements
///////////////////////////////////////////////////////////////////////////

pub trait Element {
    fn render(&mut self) -> Mesh;
}

pub struct Node {
    pub id: String,
    pub body: Box<dyn Element>,
    pub parent_node: Option<froggy::Pointer<Node>>,
    pub children: Vec<froggy::Pointer<Node>>,
}


#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub transform: cgmath::Matrix4<f32>,
    pub update: bool,
}

impl Mesh {
    pub fn require_update(&self) -> bool {
        self.update
    }
}
