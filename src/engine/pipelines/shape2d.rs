use crate::definitions::{Vertex, OPENGL_TO_WGPU_MATRIX};
use crate::engine::{load_glsl, Frame, ShaderStage};
use cgmath::{self, prelude::*};
use wgpu;

///////////////////////////////////////////////////////////////////////////
// Pipeline
///////////////////////////////////////////////////////////////////////////

pub struct Pipeline {
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_buf: wgpu::Buffer,
    pub ortho_matrix: cgmath::Matrix4<f32>,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Pipeline {
        let ortho_matrix = Self::generate_matrix(sc_desc.width as f32, sc_desc.height as f32);
        let ortho_buffer: &[f32; 16] = ortho_matrix.as_ref();

        let default_transform: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
        let transform_buf: &[f32; 16] = default_transform.as_ref();

        let uniform_buf = device
            .create_buffer_mapped(16, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
            .fill_from_slice(ortho_buffer);

        let default_transform = device
            .create_buffer_mapped(16, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
            .fill_from_slice(transform_buf);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0..64,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &default_transform,
                        range: 0..64,
                    },
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let vs_bytes = load_glsl(include_str!("shaders/shader.vert"), ShaderStage::Vertex);
        let fs_bytes = load_glsl(include_str!("shaders/shader.frag"), ShaderStage::Fragment);

        let vs_module = device.create_shader_module(&vs_bytes);
        let fs_module = device.create_shader_module(&fs_bytes);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
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
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Pipeline {
            bind_group,
            render_pipeline: pipeline,
            uniform_buf,
            ortho_matrix,
        }
    }

    fn generate_matrix(width: f32, height: f32) -> cgmath::Matrix4<f32> {
        let matrix: cgmath::Matrix4<f32> = cgmath::Ortho::<f32> {
            left: 0.0,
            right: width,
            bottom: height,
            top: 0.0,
            near: -1.0,
            far: 1.0,
        }
        .into();
        OPENGL_TO_WGPU_MATRIX * matrix
    }

    pub fn render(
        &mut self,
        frame: &Frame,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.output().view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: frame.get_clear(),
            }],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&self.render_pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);

        for mesh in frame.entries() {
            let vbo = device
                .create_buffer_mapped(mesh.vertices.len(), wgpu::BufferUsage::VERTEX)
                .fill_from_slice(&mesh.vertices);

            pass.set_vertex_buffers(0, &[(&vbo, 0)]);

            let index_buf = device
                .create_buffer_mapped(mesh.indices.len(), wgpu::BufferUsage::INDEX)
                .fill_from_slice(&mesh.indices);

            pass.set_index_buffer(&index_buf, 0);
            pass.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);
        }
    }
}
