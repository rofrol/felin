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
pub struct Mesh<T: Clone> {
    pub vertices: Vec<T>,
    pub indices: Vec<u16>,
}

impl<T: Clone> MeshTrait<T> for Mesh<T> {
    fn get_indices(&mut self) -> Vec<u16> {
        self.indices.clone()
    }
    fn get_vertices(&mut self) -> Vec<T> {
        self.vertices.clone()
    }
}

pub trait MeshTrait<T> {
    fn get_indices(&mut self) -> Vec<u16>;
    fn get_vertices(&mut self) -> Vec<T>;
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

#[derive(Clone, Copy, Debug)]
pub struct Instance {
    pub translate: cgmath::Vector2<f32>,
    pub scale: cgmath::Vector2<f32>,
    pub color: [f32; 4],
}

pub trait ElementCore: Sized {
    type Vertex;
    fn build(&mut self) -> Option<Self>;
    fn get_style(&self) -> Style;
    fn get_id(&self) -> Option<String>;
    fn set_style(&mut self, style: Style);
    fn mesh(&self) -> Mesh<Vertex>;
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
