use felin::mesh::{Image, Rectangle, Text};
use felin::{app, pipeline, utils::FontPallet, Base, Event, System};

use winit::{dpi::LogicalSize, window::WindowBuilder};

pub struct Main {
    pipeline: pipeline::default::Pipeline,
    font_texture: wgpu::BindGroup,
    rec: Image,
    rec2: Text,
}

impl Base for Main {
    fn init(system: &mut System) -> (Self, winit::window::WindowBuilder) {

        let window = WindowBuilder::new()
            .with_title("title")
            .with_resizable(true);

        let font_data = include_bytes!("./assets/Roboto.ttf");
        let font: FontPallet = FontPallet::new(120, font_data).cache_asciis();

        let mut pipeline = pipeline::default::Pipeline::new(system);
        let font_texture = pipeline.create_font_texture(system, &font);

        let rec = Image::new()
            .width(1800.0)
            .height(1000.0)
            .x(55.0)
            .y(55.0)
            .use_texture(0)
            .build();

        let rec2 = Text::new()
            .width(130.0)
            .height(200.0)
            .x(350.0)
            .y(350.0)
            .build(&font);

        (
            Main {
                pipeline,
                font_texture,
                rec,
                rec2,
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
                    clear_color: wgpu::Color::BLACK,
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
                &self.rec2.mesh().indices,
                &self.rec2.mesh().vertices,
                Some(&self.font_texture),
            );
        }
        system.queue.submit(&[encoder.finish()]);
    }
}

fn main() {
    app::<Main>()
}
