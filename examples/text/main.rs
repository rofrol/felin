use felin::mesh::{Image, Text};
use felin::{app, pipeline, utils::FontPallet, Base, Event, System};

use winit::{dpi::LogicalSize, window::WindowBuilder};

pub struct Main {
    pipeline: pipeline::default::Pipeline,
    font_texture: wgpu::BindGroup,
    text_container: Text,
    rec: Image,
}

impl Base for Main {
    fn init(system: &mut System) -> (Self, winit::window::WindowBuilder) {
        let window = WindowBuilder::new()
            .with_title("title")
            .with_resizable(true);


        let font: FontPallet = FontPallet::new(24, include_bytes!("./assets/Roboto.ttf")).cache_asciis();

        let mut pipeline = pipeline::default::Pipeline::new(system);
        let font_texture = pipeline.create_font_texture(system, &font);

        let text_container = Text::new()
            .width(530.0)
            .height(500.0)
            .text("R")
            .x(350.0)
            .y(350.0)
            .build(&font);

        let rec = Image::new()
            .width(font.max_w as f32)
            .height(font.max_h as f32)
            .x(55.0)
            .y(55.0)
            .use_texture(0)
            .build();

        (
            Main {
                pipeline,
                font_texture,
                text_container,
                rec,
            },
            window,
        )
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
                system,
                &self.rec.mesh().indices,
                &self.rec.mesh().vertices,
                Some(&self.font_texture),
            );

            self.pipeline.draw(
                &mut pass,
                system,
                &self.text_container.mesh().indices,
                &self.text_container.mesh().vertices,
                Some(&self.font_texture),
            );
        }
        system.queue.submit(&[encoder.finish()]);
    }
}

fn main() {
    app::<Main>()
}