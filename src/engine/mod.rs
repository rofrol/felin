use winit::{
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::definitions::{RenderResult, Vertex};
pub use window_events::Event;

pub mod shape2d;
pub mod window_events;

///////////////////////////////////////////////////////////////////////////
// Base trait for application
///////////////////////////////////////////////////////////////////////////
pub trait Base: 'static {
    fn init(window: &mut Window) -> Self;
    fn update(&mut self, event: &Event);
    fn render(&mut self, window: &mut Window, rpass: &mut RenderPass);
}

#[allow(dead_code)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

pub fn load_glsl(code: &str, stage: ShaderStage) -> Vec<u32> {
    let ty = match stage {
        ShaderStage::Vertex => glsl_to_spirv::ShaderType::Vertex,
        ShaderStage::Fragment => glsl_to_spirv::ShaderType::Fragment,
        ShaderStage::Compute => glsl_to_spirv::ShaderType::Compute,
    };

    wgpu::read_spirv(glsl_to_spirv::compile(&code, ty).unwrap()).unwrap()
}

///////////////////////////////////////////////////////////////////////////
// RenderPass
///////////////////////////////////////////////////////////////////////////
pub struct RenderPass<'a, 'b> {
    pub pass: wgpu::RenderPass<'a>,
    pub device: &'b wgpu::Device,
}

impl<'a, 'b> RenderPass<'a, 'b> {
    pub fn create(
        frame: &wgpu::SwapChainOutput,
        encoder: &'a mut wgpu::CommandEncoder,
        device: &'b wgpu::Device,
    ) -> Self {
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color::RED,
            }],
            depth_stencil_attachment: None,
        });

        RenderPass { pass, device }
    }

    pub fn setup(&mut self, pipeline: &wgpu::RenderPipeline, bind_group: &wgpu::BindGroup) {
        self.pass.set_pipeline(&pipeline);
        self.pass.set_bind_group(0, &bind_group, &[]);
    }

    pub fn draw(&mut self, vertices: Vec<Vertex>) {
        let vbo = self
            .device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertices);

        self.pass.set_vertex_buffers(0, &[(&vbo, 0)]);
        self.pass.draw(0..vertices.len() as u32, 0..1);
    }

    pub fn draw_indexed(&mut self, vertices: Vec<Vertex>, indices: Vec<u16>) {
        let vbo = self
            .device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertices);

        self.pass.set_vertex_buffers(0, &[(&vbo, 0)]);

        let index_buf = self
            .device
            .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&indices);

        self.pass.set_index_buffer(&index_buf, 0);
        self.pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    pub fn draw_result(&mut self, mesh: RenderResult) {
        if mesh.indices.len() > 0 {
            self.draw_indexed(mesh.vertices, mesh.indices);
        } else {
            self.draw(mesh.vertices);
        }
    }
}

///////////////////////////////////////////////////////////////////////////
// Window
///////////////////////////////////////////////////////////////////////////

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub clear_color: wgpu::Color,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            clear_color: wgpu::Color::GREEN,
        }
    }

    pub fn update_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_color(&mut self, color: [f64; 4]) {
        self.clear_color = wgpu::Color {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3],
        }
    }
}

///////////////////////////////////////////////////////////////////////////
// Main Application logic
///////////////////////////////////////////////////////////////////////////

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }

    pub fn init<E: Base>(&mut self, title: &str) {
        env_logger::init();
        let window_event_loop = EventLoop::new();

        #[cfg(not(feature = "gl"))]
        let (_window, instance, hidpi_factor, size, surface) = {
            let window = WindowBuilder::new()
                .with_title(title)
                .with_resizable(true)
                .build(&window_event_loop)
                .unwrap();
            let hidpi_factor = window.hidpi_factor();
            let size = window.inner_size().to_physical(hidpi_factor);

            let instance = wgpu::Instance::new();
            let surface = instance.create_surface(&window);

            (window, instance, hidpi_factor, size, surface)
        };

        #[cfg(feature = "gl")]
        let (_window, instance, hidpi_factor, size, surface) = {
            let wb = winit::WindowBuilder::new();
            let cb = wgpu::glutin::ContextBuilder::new().with_vsync(true);
            let context = cb.build_windowed(wb, &event_loop).unwrap();
            context.window().set_title(title);

            let hidpi_factor = context.window().hidpi_factor();
            let size = context
                .window()
                .get_inner_size()
                .unwrap()
                .to_physical(hidpi_factor);

            let (context, window) = unsafe { context.make_current().unwrap().split() };

            let instance = wgpu::Instance::new(context);
            let surface = instance.get_surface();

            (window, instance, hidpi_factor, size, surface)
        };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
        });

        let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let mut sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width.round() as u32,
            height: size.height.round() as u32,
            present_mode: wgpu::PresentMode::Vsync,
        };

        let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let default_pipeline: shape2d::Pipeline = shape2d::Pipeline::new(&device, &sc_desc);
        let mut window = Window::new(sc_desc.width, sc_desc.height);
        let mut input = Event::new();
        let mut example = E::init(&mut window);

        ///////////////////////////////////////////////////////////////////////////
        // Render loop
        ///////////////////////////////////////////////////////////////////////////

        window_event_loop.run(move |event, _, control_flow| {
            *control_flow = if cfg!(feature = "metal-auto-capture") {
                ControlFlow::Exit
            } else {
                ControlFlow::Poll
            };
            match event {
                event::Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let physical = size.to_physical(hidpi_factor);
                    sc_desc.width = physical.width.round() as u32;
                    sc_desc.height = physical.height.round() as u32;
                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                    window.update_size(sc_desc.width, sc_desc.height);
                }
                event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {
                        input.handle_event(event);
                        example.update(&input)
                    }
                },
                event::Event::EventsCleared => {
                    let frame = swap_chain.get_next_texture();
                    let mut encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

                    {
                        let mut render_pass: RenderPass =
                            RenderPass::create(&frame, &mut encoder, &device);

                        render_pass.setup(
                            &default_pipeline.render_pipeline,
                            &default_pipeline.bind_group,
                        );
                        example.render(&mut window, &mut render_pass);
                    }

                    device.get_queue().submit(&[encoder.finish()]);
                }
                _ => (),
            }
        });
    }
}
