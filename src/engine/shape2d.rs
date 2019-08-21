use wgpu;
use cgmath;
use std::{mem};
use cgmath::prelude::*;
use crate::engine::{ShaderStage, load_glsl};
use crate::gui::definitions::{Vertex, OPENGL_TO_WGPU_MATRIX};
use crate::gui::{Registry};

#[derive(Clone, Copy, Debug)]
pub struct UniformBufferObject {
    pub proj: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub transform: [[f32; 4]; 4],
}

///////////////////////////////////////////////////////////////////////////
// Pipeline
///////////////////////////////////////////////////////////////////////////

pub struct Pipeline {
   pub bind_group: wgpu::BindGroup, 
   pub render_pipeline: wgpu::RenderPipeline,
   pub uniform_buf: wgpu::Buffer,
   pub matrixObject: UniformBufferObject,
   pub registry: Registry,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Pipeline {
          
        let matrix = Self::generate_matrix(sc_desc.width as f32 / sc_desc.height as f32);

        let buffer_size = mem::size_of::<UniformBufferObject>() as wgpu::BufferAddress;

        let uniform_buf = device
                .create_buffer_mapped(
                    1,
                    wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::TRANSFER_DST,
                )
                .fill_from_slice(&[matrix]);

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer,
                },
            ] }
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0 .. buffer_size,
                    },
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let vs_bytes = load_glsl(include_str!("shaders/shader2.vert"), ShaderStage::Vertex);
        let fs_bytes = load_glsl(include_str!("shaders/shader2.frag"), ShaderStage::Fragment);

        let vs_module = device.create_shader_module(&vs_bytes);
        let fs_module = device.create_shader_module(&fs_bytes);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::PipelineStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::PipelineStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            },
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8Unorm,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
              vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Float2,
                        offset: 0,
                        shader_location: 0,
                    },
                    wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Float4,
                        offset: 2 * 4,
                        shader_location: 1,
                    },
                ],
            }],
            sample_count: 1,
        });

        Pipeline {
            bind_group,
            render_pipeline: pipeline,
            uniform_buf,
            matrixObject: matrix,
            registry: Registry::new(),
        }
    }

    fn generate_matrix(aspect_ratio: f32) -> UniformBufferObject {
        let mx_projection = cgmath::perspective(cgmath::Deg(45f32), aspect_ratio, 1.0, 10.0);
        let mx_view = cgmath::Matrix4::look_at(
            cgmath::Point3::new(1.5f32, 0.0, 5.0),
            cgmath::Point3::new(0f32, 0.0, 0.0),
            cgmath::Vector3::unit_z(),
        );

        let transform = cgmath::Matrix4::identity();
        let projection = OPENGL_TO_WGPU_MATRIX * mx_projection;
        

        UniformBufferObject {
            proj: *projection.as_ref(),
            view: *mx_view.as_ref(),
            transform: *transform.as_ref(),
        }
    }

    pub fn updateMatrix(&mut self, aspect_ratio: f32) {
        let mx_projection = cgmath::perspective(cgmath::Deg(45f32), aspect_ratio, 1.0, 10.0);
        let mx_view = cgmath::Matrix4::look_at(
            cgmath::Point3::new(1.5f32, 0.0, 5.0),
            cgmath::Point3::new(0f32, 0.0, 0.0),
            cgmath::Vector3::unit_z(),
        );

        let transform = cgmath::Matrix4::identity();
        let projection = OPENGL_TO_WGPU_MATRIX * mx_projection;
        
        self.matrixObject = UniformBufferObject {
            proj: *projection.as_ref(),
            view: *mx_view.as_ref(),
            transform: *transform.as_ref(),
        };
    }


    pub fn draw(&self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
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

                for value in self.registry.entries.iter_all() {
                    let vertices = value.body.render();

                    let vbo = device
                        .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
                        .fill_from_slice(&vertices); 

                    rpass.set_pipeline(&self.render_pipeline);
                    rpass.set_bind_group(0, &self.bind_group, &[]);
                    rpass.set_vertex_buffers(&[(&vbo, 0)]);
                    rpass.draw(0 .. vertices.len() as u32, 0 .. 1);                   
                }
            }

            device.get_queue().submit(&[encoder.finish()]);
    }
}

