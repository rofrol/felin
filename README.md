# wgpu-2d

This is an exercise repo for myself to learn how to handle vulkan and 2D graphics, it use's wgpu-rs as graphics backend.


To try it, include master branch on your cargo

```rust
[dependencies.engine]
{ git = "https://github.com/tonis2/wgpu-2d", branch = "master" }

```


```rust
use engine::{Base, App, wgpu, shape2d};
use engine::gui::definitions::*;


pub struct Main {
    pipeline: shape2d::Pipeline,
}

impl Base for Main {
    
    fn init(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self {
        let pipeline = shape2d::Pipeline::new(&device);
       
        Main {
            pipeline,
        }
    }

    fn update(&mut self, _event: wgpu::winit::WindowEvent) {}
    fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) {}

    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
        

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

            {
              let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::GREEN,
                    }],
                    depth_stencil_attachment: None,
                });

                let mut vertex_data = vec!(
                                            Vertex::new([-0.50, 0.0], [1.0, 1.0, 1.0]),
                                            Vertex::new([0.0, -0.50], [0.0, 1.0, 0.0]),
                                            Vertex::new([0.0, -0.20], [0.5, 0.5, 1.0]),
                                            
                                            Vertex::new([0.50, 0.0], [1.0, 1.0, 1.0]),
                                            Vertex::new([0.0, 0.50], [0.0, 1.0, 0.0]),
                                            Vertex::new([0.0, 0.20], [0.5, 0.5, 1.0]),
                
                                           );
                
                let vbo = device
                    .create_buffer_mapped(vertex_data.len(), wgpu::BufferUsage::VERTEX)
                    .fill_from_slice(&vertex_data);
           
                rpass.set_pipeline(&self.pipeline.render_pipeline);
                rpass.set_bind_group(0, &self.pipeline.bind_group, &[]);
                rpass.set_vertex_buffers(&[(&vbo, 0)]);
                rpass.draw(0 .. vertex_data.len() as u32, 0 .. 1);
                
            }

            device.get_queue().submit(&[encoder.finish()]);
    }
}

fn main() {
    App::new::<Main>("Title");
}


```
