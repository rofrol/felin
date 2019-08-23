
use log::info;
use wgpu::winit::{
    ElementState,
    Event,
    EventsLoop,
    KeyboardInput,
    VirtualKeyCode,
    WindowEvent,
};

use crate::gui;
use crate::definitions::{Vertex, RenderResult};

pub mod shape2d;

#[allow(dead_code)]
pub fn cast_slice<T>(data: &[T]) -> &[u8] {
    use std::mem::size_of;
    use std::slice::from_raw_parts;

    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

#[allow(dead_code)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

pub trait Base {
    fn init(custom_elements: &mut gui::ElementRegistry) -> Self;
    fn update(&mut self, event: wgpu::winit::WindowEvent, custom_elements: &mut gui::ElementRegistry);
    fn render(&mut self, rpass: &mut RenderPass, custom_elements: &mut gui::ElementRegistry);
}

pub fn load_glsl(code: &str, stage: ShaderStage) -> Vec<u8> {
    use std::io::Read;

    let ty = match stage {
        ShaderStage::Vertex => glsl_to_spirv::ShaderType::Vertex,
        ShaderStage::Fragment => glsl_to_spirv::ShaderType::Fragment,
        ShaderStage::Compute => glsl_to_spirv::ShaderType::Compute,
    };

    let mut output = glsl_to_spirv::compile(&code, ty).unwrap();
    let mut spv = Vec::new();
    output.read_to_end(&mut spv).unwrap();
    spv
}


pub struct RenderPass<'a, 'b> {
    pub pass: wgpu::RenderPass<'a>,
    pub device: &'b wgpu::Device,
}

impl <'a, 'b>RenderPass<'a, 'b> {
    pub fn create(frame: &wgpu::SwapChainOutput, encoder: &'a mut wgpu::CommandEncoder, device: &'b wgpu::Device) -> Self {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::GREEN,
                }],
                depth_stencil_attachment: None,
            });

        RenderPass {
            pass,
            device,
        }
    }

    pub fn setup(&mut self, pipeline: &wgpu::RenderPipeline, bind_group: &wgpu::BindGroup) {
        self.pass.set_pipeline(&pipeline); 
        self.pass.set_bind_group(0, &bind_group, &[]);
    }

    pub fn draw(&mut self, vertices: Vec<Vertex>) {
        let vbo = self.device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertices); 

        self.pass.set_vertex_buffers(&[(&vbo, 0)]);
        self.pass.draw(0 .. vertices.len() as u32, 0 .. 1); 
    }

    pub fn draw_indexed(&mut self, vertices: Vec<Vertex>, indices: Vec<u16>) {
        let vbo = self.device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertices); 

        self.pass.set_vertex_buffers(&[(&vbo, 0)]);

        let index_buf = self.device
            .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&indices);

        self.pass.set_index_buffer(&index_buf, 0);
        self.pass.draw_indexed(0 .. indices.len() as u32, 0, 0 .. 1);  
    }


    pub fn draw_result(&mut self, mesh: RenderResult) {
        if mesh.indices.len() > 0 {
            self.draw_indexed(mesh.vertices, mesh.indices);
        } else {
            self.draw(mesh.vertices);
        }
    }
}

pub struct App {
    custom_elements: gui::ElementRegistry,
}

impl App {
    pub fn new() -> App {
       App {
         custom_elements: gui::ElementRegistry::new(),
       }
    }

    pub fn init<E: Base>(&mut self, title: &str) {
        env_logger::init();

        let mut events_loop = EventsLoop::new();

        info!("Initializing the window...");

        #[cfg(not(feature = "gl"))]
        let (_window, instance, hidpi_factor, size, surface) = {
            use wgpu::winit::WindowBuilder;

            let instance = wgpu::Instance::new();

            let window = WindowBuilder::new()
                .with_title(title)
                .with_maximized(true)
                .with_resizable(true)
                .build(&events_loop)
                .unwrap();

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

        let mut default_pipeline:shape2d::Pipeline = shape2d::Pipeline::new(&device, &sc_desc);

        let mut example = E::init(&mut self.custom_elements);

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
                    default_pipeline = shape2d::Pipeline::new(&device, &sc_desc);
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
                        example.update(event, &mut self.custom_elements);
                    }
                },
                _ => (),
            });

            let frame = swap_chain.get_next_texture();

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

            {
                let mut render_pass: RenderPass = RenderPass::create(&frame, &mut encoder, &device);

                render_pass.setup(&default_pipeline.render_pipeline, &default_pipeline.bind_group);

                example.render(&mut render_pass, &mut self.custom_elements);
            }

            device.get_queue().submit(&[encoder.finish()]);   
        
          
            running &= !cfg!(feature = "metal-auto-capture");
        }
    }
}
