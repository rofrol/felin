use felin::{app, pipeline, Base, Event, System};
use slider::Slider;
use winit::{dpi::LogicalSize, window::WindowBuilder};

mod slider;

pub struct Main {
    pipeline: pipeline::default::Pipeline,
    slider: Slider,
    buttons: wgpu::BindGroup,
    images: wgpu::BindGroup,
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

        system.screen_descriptor.width = window_size.width as u32;
        system.screen_descriptor.height = window_size.height as u32;

        let mut pipeline = pipeline::default::Pipeline::new(system);
        let mut slider = Slider::new(&window);
        slider.build();

        let buttons = pipeline.create_textures_array(
            system,
            vec![
                "examples/slider/assets/arrow_left.png",
                "examples/slider/assets/arrow_left_active.png",
                "examples/slider/assets/arrow_right.png",
                "examples/slider/assets/arrow_right_active.png",
            ],
        );

        let images = pipeline.create_textures_array(
            system,
            vec![
                "examples/slider/assets/image1.jpg",
                "examples/slider/assets/image2.jpg",
                "examples/slider/assets/image3.jpg",
            ],
        );

        (
            Main {
                pipeline,
                slider,
                buttons,
                images,
            },
            window,
        )
    }

    fn update(&mut self, system: &mut System, events: &Event) {
        self.slider.update(&events);
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

            self.pipeline.draw(
                &mut pass,
                &system,
                &self.slider.container.indices,
                &self.slider.container.vertices,
                Some(&self.buttons),
            );

            self.pipeline.draw(
                &mut pass,
                &system,
                &self.slider.gallery.indices,
                &self.slider.gallery.vertices,
                Some(&self.images),
            );
        }

        system.queue.submit(&[encoder.finish()]);
    }
}

fn main() {
    app::<Main>("App")
}
