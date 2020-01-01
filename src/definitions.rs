use crate::mesh::{Circle, Image, Rectangle, Text};

///////////////////////////////////////////////////////////////////////////
// Vertex
///////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub in_position: [f32; 2],
    pub in_color: [f32; 4],
    pub tex_pos: [f32; 2],
    pub texture_id: i32,
}


#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

#[derive(Debug)]
pub struct Texture {
    pub texture_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub name: String,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Instance {
    pub translate: cgmath::Vector2<f32>,
    pub scale: cgmath::Vector2<f32>,
    pub color: [f32; 4],
}

#[derive(Clone)]
pub enum Elements {
    Circle(Circle),
    Image(Image),
    Rectangle(Rectangle),
    Text(Text),
}
