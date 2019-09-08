use crate::definitions::Mesh;
use wgpu;

pub struct Frame<'a> {
    clear_color: wgpu::Color,
    frame: &'a wgpu::SwapChainOutput<'a>,
    data: Vec<Mesh>,
}

impl<'a> Frame<'a> {
    pub fn new(frame: &'a wgpu::SwapChainOutput) -> Self {
        Frame {
            clear_color: wgpu::Color::GREEN,
            data: Vec::new(),
            frame,
        }
    }

    pub fn draw(&mut self, result: Mesh) {
        self.data.push(result)
    }

    pub fn get_clear(&self) -> wgpu::Color {
        self.clear_color
    }

    pub fn clear(&mut self, color: [f64; 4]) {
        self.clear_color = wgpu::Color {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3],
        }
    }

    pub fn output(&self) -> &wgpu::SwapChainOutput {
        self.frame
    }

    pub fn entries(&self) -> Vec<Mesh> {
        self.data.clone()
    }
}
