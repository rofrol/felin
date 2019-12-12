pub mod definitions;
pub mod events;
pub mod mesh;
pub mod pipeline;
pub mod system;
pub mod utils;


pub use events::Event;
pub use system::System;

use winit::{
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

///////////////////////////////////////////////////////////////////////////
// Base trait for application
///////////////////////////////////////////////////////////////////////////
pub trait Base: 'static + Sized {
    fn init(system: &mut System) -> Self;
    fn update(&mut self, system: &mut System, events: &Event);
    fn render(&mut self, swap_chain: &mut wgpu::SwapChain, system: &mut System);
}

///////////////////////////////////////////////////////////////////////////
// Main Application logic
///////////////////////////////////////////////////////////////////////////

pub fn app<E: Base>() {
    let window_event_loop = EventLoop::new();

    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        backends: wgpu::BackendBit::PRIMARY,
    })
    .unwrap();

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: 0,
        height: 0,
        present_mode: wgpu::PresentMode::Vsync,
    };

    let mut system = System {
        device,
        screen_descriptor: sc_desc,
        queue,
        window: WindowBuilder::new()
        .with_title("title")
        .with_resizable(true),
    };

    let mut example = E::init(&mut system);

    let window = system.window.clone()
        .build(&window_event_loop)
        .unwrap();

    let hidpi_factor = window.hidpi_factor();
    let size = window.inner_size().to_physical(hidpi_factor);
    let surface = wgpu::Surface::create(&window);

    system.screen_descriptor.width = size.width.round() as u32;
    system.screen_descriptor.height = size.height.round() as u32;

    let mut swap_chain = system.device.create_swap_chain(&surface, &system.screen_descriptor);
    let mut input_events = Event::new();

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
                input_events.dpi_factor = hidpi_factor;
                input_events.resized = true;
                system.screen_descriptor.width = physical.width.round() as u32;
                system.screen_descriptor.height = physical.height.round() as u32;
                swap_chain = system
                    .device
                    .create_swap_chain(&surface, &system.screen_descriptor);
            }
            event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    input_events.handle_event(event);
                    example.update(&mut system, &input_events);
                    input_events.clear();
                }
            },
            event::Event::EventsCleared => {
                example.render(&mut swap_chain, &mut system);
            }
            _ => (),
        }
    });
}
