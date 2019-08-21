use crate::gui::definitions::{Vertex, Element};
use wgpu;

pub struct Triangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Triangle {
    pub fn new() -> Self {
        Triangle {
            x: 10.0,
            y: 50.0,
            width: 100.0,
            height: 50.0,
        }
    }
}

impl Element for Triangle { 
    fn render(&self, rpass: &mut wgpu::RenderPass, device: &mut wgpu::Device) {
            let vertex_data = vec!(
                    Vertex::new([-0.50, 0.0], [1.0, 1.0, 1.0]),
                    Vertex::new([0.0, -0.50], [1.0, 1.0, 1.0]),
                    Vertex::new([0.0, -0.20], [1.0, 1.0, 1.0]),
            );

            let vbo = device
                .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
                .fill_from_slice(&vertex_data); 

            rpass.set_vertex_buffers(&[(&vbo, 0)]);
            rpass.draw(0 .. vertex_data.len() as u32, 0 .. 1);   
    }
}