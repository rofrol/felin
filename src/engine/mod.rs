use winit::{
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub use frame::Frame;
use pipelines::shape2d;
pub use window_events::Event;

mod frame;
mod pipelines;
pub mod window_events;

///////////////////////////////////////////////////////////////////////////
// Base trait for application
///////////////////////////////////////////////////////////////////////////
pub trait Base: 'static {
    fn init(window: &mut Window) -> Self;
    fn update(&mut self, event: &Event);
    fn render(&mut self, window: &mut Window, frame: &mut Frame);
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

pub fn App<E: Base>(title: &str) {
    env_logger::init();
    let window_event_loop = EventLoop::new();

    #[cfg(not(feature = "gl"))]
    let (_window, hidpi_factor, size, surface) = {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_resizable(true)
            .build(&window_event_loop)
            .unwrap();
        let hidpi_factor = window.hidpi_factor();
        let size = window.inner_size().to_physical(hidpi_factor);
        let surface = wgpu::Surface::create(&window);

        (window, hidpi_factor, size, surface)
    };

    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        backends: wgpu::BackendBit::PRIMARY,
    })
    .unwrap();

    let (device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
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

    let mut default_pipeline: shape2d::Pipeline = shape2d::Pipeline::new(&device, &sc_desc);
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
                let frame_texture = swap_chain.get_next_texture();
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
                let mut frame = Frame::new(&frame_texture);

                {
                    example.render(&mut window, &mut frame);
                }

                default_pipeline.render(&frame, &mut encoder, &device);
                queue.submit(&[encoder.finish()]);
            }
            _ => (),
        }
    });
}
