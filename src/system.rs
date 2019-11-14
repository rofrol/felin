use crate::utils::OPENGL_TO_WGPU_MATRIX;

pub struct System {
    pub device: wgpu::Device,
    pub screen_descriptor: wgpu::SwapChainDescriptor,
    pub queue: wgpu::Queue,
    pub window: winit::window::Window,
}

impl System {
    pub fn get_screen_matrix(&self) -> cgmath::Matrix4<f32> {
        let matrix: cgmath::Matrix4<f32> = cgmath::Ortho::<f32> {
            left: 0.0,
            right: self.screen_descriptor.width as f32,
            bottom: self.screen_descriptor.height as f32,
            top: 0.0,
            near: -1.0,
            far: 1.0,
        }
        .into();
        OPENGL_TO_WGPU_MATRIX * matrix
    }
}
