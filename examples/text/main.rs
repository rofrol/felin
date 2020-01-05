use felin::mesh::Text;
use felin::prelude::*;
use felin::{
    app, pipeline,
    utils::{font::FontPallet, Style},
    Base, Event, System,
};
use winit::{dpi::LogicalSize, window::WindowBuilder};

pub struct Main {
    text_pipeline: pipeline::text::Pipeline,
    font_texture: wgpu::BindGroup,
    text_container: Text,
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

        FontPallet::load_font("examples/text/assets/Roboto.ttf");

        let mut text_pipeline = pipeline::text::Pipeline::new(system);
        let font_texture =
            text_pipeline.create_font_texture(system, &FontPallet::create_font("Roboto", 32));

        let mut text_container = Text {
            style: Style::default(),
            font: "Roboto".to_string(),
            text: "Tere olen tonis !".to_string(),
            ..Default::default()
        };

        text_container.build();

        Main {
            text_pipeline,
            font_texture,
            text_container,
        }
    }

    fn update(&mut self, system: &mut System, events: &Event) {
        if events.resized {
            self.text_pipeline.resize(system);
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

            self.text_pipeline.draw(
                &mut pass,
                system,
                &self.text_container.mesh().indices,
                &self.text_container.mesh().vertices,
                &self.font_texture,
            );
        }
        system.queue.submit(&[encoder.finish()]);
    }
}

fn main() {
    app::<Main>()
}
