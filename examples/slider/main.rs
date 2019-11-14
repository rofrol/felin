use felin::{app, pipeline, Base, Event, System};

pub struct Main {
    pipeline: pipeline::default::Pipeline,
}

impl Base for Main {
    fn init(system: &mut System) -> Self {
        let mut pipeline = pipeline::default::Pipeline::new(system);
        let images = pipeline.create_textures_array(
            system,
            vec![
                "examples/slider/assets/arrow_left.png",
                "examples/slider/assets/arrow_right.png",
                "examples/slider/assets/image1.jpg",
                "examples/slider/assets/image2.jpg",
                "examples/slider/assets/image3.jpg",
                "examples/slider/assets/image4.jpg",
            ],
        );

        Main { pipeline }
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
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });
        }

        system.queue.submit(&[encoder.finish()]);
    }
}

fn main() {
    app::<Main>("App")
}
