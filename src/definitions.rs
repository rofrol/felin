use crate::utils::font::FontPallet;
use crate::utils::Style;
use collision::Aabb2;

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

pub trait ElementCore {
    fn build(&mut self);
    fn get_style(&self) -> Style;
    fn get_id(&self) -> Option<String>;
    fn set_style(&mut self, style: Style);
    fn mesh(&mut self) -> Mesh;
}

pub trait ElememtResizable {
    fn width(&mut self, width: f32);
    fn height(&mut self, height: f32);
    fn radius(&mut self, radius: f32);
}

pub trait ElementImageBuild {
    fn build(&mut self, font: &FontPallet);
}

pub trait ElementCollider {
    fn contains(&self, point: cgmath::Point2<f32>) -> bool;
    fn get_collider(&self) -> Aabb2<f32>;
}
