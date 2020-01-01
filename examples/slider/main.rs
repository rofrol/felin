use felin::prelude::*;
use felin::{app, pipeline, utils::FontPallet, Base, Event, System};
use winit::{dpi::LogicalSize, window::WindowBuilder};
mod slider;

pub struct Main {
    pipeline: pipeline::default::Pipeline,
    slider: slider::Element,
}

impl Base for Main {
    fn init(system: &mut System) -> Self {
        system.set_window(
            WindowBuilder::new()
                .with_title("title")
                .with_inner_size(LogicalSize {
                    width: 1500.0,
                    height: 800.0,
                })
                .with_resizable(true),
        );

        let pipeline = pipeline::default::Pipeline::new(system);
        let slider = slider::Element::new();

        Main { pipeline, slider }
    }

    fn update(&mut self, system: &mut System, events: &Event) {
        if events.resized {
            self.pipeline.resize(system);
        };
    }

    fn render(&mut self, swap_chain: &mut wgpu::SwapChain, system: &mut System) {
        let frame_texture = swap_chain.get_next_texture();
        let mut encoder = system
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame_texture.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    },
                }],
                depth_stencil_attachment: None,
            });

            self.pipeline.draw(
                &mut pass,
                &system,
                &self.slider.container.indices,
                &self.slider.container.vertices,
                None,
            );
        }
        system.queue.submit(&[encoder.finish()]);
    }
}

fn main() {
    app::<Main>()
}
