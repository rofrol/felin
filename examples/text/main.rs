use felin::mesh::Text;
use felin::{app, pipeline, utils::FontPallet, Base, Event, System};

use winit::{dpi::LogicalSize, window::WindowBuilder};

pub struct Main {
    pipeline: pipeline::default::Pipeline,
}

impl Base for Main {
    fn init(system: &mut System) -> (Self, winit::window::WindowBuilder) {
        let window_size = LogicalSize {
            width: 1500.0,
            height: 800.0,
        };
        let window = WindowBuilder::new()
            .with_title("title")
            .with_inner_size(window_size)
            .with_resizable(true);

        let font: FontPallet =
            FontPallet::new(12).cache_asciis(include_bytes!("./assets/Roboto.ttf"));

        let mut pipeline = pipeline::default::Pipeline::new(system);

        (Main { pipeline }, window)
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
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 0.0,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        system.queue.submit(&[encoder.finish()]);
    }
}

fn main() {
    app::<Main>()
}
