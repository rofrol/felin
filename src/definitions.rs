use crate::utils::font::FontPallet;
use collision::Aabb2;
use std::cell::RefCell;
use std::rc::Rc;

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
    fn x(&mut self, x: f32);
    fn y(&mut self, y: f32);
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn color(&mut self, color: [f32; 4]);
    fn mesh(&mut self) -> Mesh;
    fn build(&mut self);
    fn is_resizable(&mut self) -> Option<&mut dyn ElememtResizable>;
    fn as_rc(&mut self) -> Rc<RefCell<dyn ElementCore>>;
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

pub trait ElementImage {
    fn use_texture(&mut self, index: i32);
}
