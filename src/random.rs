
pub struct Main;

impl Main {

    fn init(sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) -> Self { 
        use std::mem;
        let mut init_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        let mx_total = Self::generate_matrix(sc_desc.width as f32 / sc_desc.height as f32);
        let mx_ref: &[f32; 16] = mx_total.as_ref();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        encoder.copy_buffer_to_buffer(&temp_buf, 0, &self.uniform_buf, 0, 64);
        device.get_queue().submit(&[encoder.finish()]);
    }

    fn update(&mut self, _event: wgpu::winit::WindowEvent) {
        //empty
    }

    fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device) {

    }


    fn render(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(&self.index_buf, 0);
            rpass.set_vertex_buffers(&[(&self.vertex_buf, 0)]);
            rpass.draw_indexed(0 .. self.index_count as u32, 0, 0 .. 1);
        }

        device.get_queue().submit(&[encoder.finish()]);
    }

}




