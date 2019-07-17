mod engine;

pub use engine::Base;
use log::info;
use wgpu::winit::{
    ElementState,
    Event,
    EventsLoop,
    KeyboardInput,
    VirtualKeyCode,
    WindowEvent,
};

pub use wgpu;
pub use engine::shape2d;


pub struct App;

impl App {
    pub fn new<E: Base>(title: &str) -> App {
 
        env_logger::init();

        let mut events_loop = EventsLoop::new();

        info!("Initializing the window...");

        #[cfg(not(feature = "gl"))]
        let (_window, instance, hidpi_factor, size, surface) = {
            use wgpu::winit::Window;

            let instance = wgpu::Instance::new();

            let window = Window::new(&events_loop).unwrap();
            window.set_title(title);
            let hidpi_factor = window.get_hidpi_factor();
            let size = window.get_inner_size().unwrap().to_physical(hidpi_factor);

            let surface = instance.create_surface(&window);

            (window, instance, hidpi_factor, size, surface)
        };

        #[cfg(feature = "gl")]
        let (instance, hidpi_factor, size, surface) = {
            let wb = wgpu::winit::WindowBuilder::new();
            let cb = wgpu::glutin::ContextBuilder::new().with_vsync(true);
            let context = wgpu::glutin::WindowedContext::new_windowed(wb, cb, &events_loop).unwrap();
            context.window().set_title(title);

            let hidpi_factor = context.window().get_hidpi_factor();
            let size = context
                .window()
                .get_inner_size()
                .unwrap()
                .to_physical(hidpi_factor);

            let instance = wgpu::Instance::new(context);
            let surface = instance.get_surface();

            (instance, hidpi_factor, size, surface)
        };

        let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
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
        };

        let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

        info!("Initializing the example...");
        let mut example = E::init(&sc_desc, &mut device);

        info!("Entering render loop...");
        let mut running = true;
        while running {
            events_loop.poll_events(|event| match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    let physical = size.to_physical(hidpi_factor);
                    info!("Resizing to {:?}", physical);
                    sc_desc.width = physical.width.round() as u32;
                    sc_desc.height = physical.height.round() as u32;
                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                    example.resize(&sc_desc, &mut device);
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    }
                    | WindowEvent::CloseRequested => {
                        running = false;
                    }
                    _ => {
                        example.update(event);
                    }
                },
                _ => (),
            });

            let frame = swap_chain.get_next_texture();
            example.render(&frame, &mut device);
            running &= !cfg!(feature = "metal-auto-capture");
        }

        App {}
    }
}



