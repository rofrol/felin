use crate::definitions::{Instance, Vertex};
use crate::utils::{load_glsl, ShaderStage};
use crate::System;
use cgmath::{self, prelude::*};

const MAX_INSTANCES: usize = 100000;

#[allow(dead_code)]
pub struct Pipeline {
    ortho_matrix_bind: wgpu::BindGroup,
    transform_matrix_bind: wgpu::BindGroup,

    ortho_matrix_buffer: wgpu::Buffer,
    transform_matrix_buffer: wgpu::Buffer,

    index_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,

    indices: Vec<u16>,
    vertices: Vec<Vertex>,
    instance_count: usize,

    render_pipeline: wgpu::RenderPipeline,
}

#[allow(dead_code)]
impl Pipeline {
    pub fn new(system: &mut System) -> Pipeline {
        let matrix = system.get_screen_matrix();
        let ortho_buffer: &[f32; 16] = matrix.as_ref();

        let default_transform: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
        let transform_buffer: &[f32; 16] = default_transform.as_ref();

        let ortho_matrix_buffer = system
            .device
            .create_buffer_mapped(16, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
            .fill_from_slice(ortho_buffer);

        let transform_matrix_buffer = system
            .device
            .create_buffer_mapped(16, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
            .fill_from_slice(transform_buffer);

        let vb_desc = system
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                }],
            });

        let ortho_matrix_bind = system.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &vb_desc,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &ortho_matrix_buffer,
                    range: 0..64,
                },
            }],
        });

        let transform_matrix_bind = system.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &vb_desc,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &transform_matrix_buffer,
                    range: 0..64,
                },
            }],
        });

        let pipeline_layout =
            system
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&vb_desc, &vb_desc],
                });

        let instance_size = std::mem::size_of::<Instance>();
        let instance_buffer = system.device.create_buffer(&wgpu::BufferDescriptor {
            size: (instance_size * MAX_INSTANCES) as u64,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });

        let vertex_description = wgpu::VertexBufferDescriptor {
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
                    offset: 1 * std::mem::size_of::<cgmath::Vector4<f32>>() as u64,
                    shader_location: 1,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 2 * std::mem::size_of::<cgmath::Vector2<f32>>() as u64,
                    shader_location: 2,
                },
            ],
        };

        let instance_description = wgpu::VertexBufferDescriptor {
            stride: instance_size as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 0,
                    shader_location: 2,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 1 * std::mem::size_of::<cgmath::Vector2<f32>>() as u64,
                    shader_location: 3,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float4,
                    offset: 2 * std::mem::size_of::<cgmath::Vector4<f32>>() as u64,
                    shader_location: 4,
                },
            ],
        };

        let vs_bytes = load_glsl(include_str!("shaders/instance.vert"), ShaderStage::Vertex);
        let fs_bytes = load_glsl(include_str!("shaders/instance.frag"), ShaderStage::Fragment);

        let vs_module = system.device.create_shader_module(&vs_bytes);
        let fs_module = system.device.create_shader_module(&fs_bytes);

        let pipeline = system
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
                color_states: &[wgpu::ColorStateDescriptor {
                    format: system.screen_descriptor.format,
                    color_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                depth_stencil_state: None,
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[vertex_description, instance_description],
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        let indices: Vec<u16> = Vec::new();
        let vertices: Vec<Vertex> = Vec::new();

        let index_buffer = system
            .device
            .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&indices);

        let vertex_buffer = system
            .device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertices);

        Pipeline {
            transform_matrix_bind,
            ortho_matrix_bind,

            ortho_matrix_buffer,
            transform_matrix_buffer,

            indices: Vec::new(),
            vertices: Vec::new(),

            vertex_buffer,
            index_buffer,
            instance_buffer,
            instance_count: 0,

            render_pipeline: pipeline,
        }
    }

    pub fn use_buffers(&mut self, system: &System, vertices: &Vec<Vertex>, indices: &Vec<u16>) {
        self.vertices = vertices.clone();
        self.indices = indices.clone();

        self.vertex_buffer = system
            .device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertices);
        self.index_buffer = system
            .device
            .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&indices);
    }
    pub fn draw(&mut self, pass: &mut wgpu::RenderPass) {
        pass.set_pipeline(&self.render_pipeline);
        pass.set_bind_group(0, &self.ortho_matrix_bind, &[]);
        pass.set_bind_group(1, &self.transform_matrix_bind, &[]);

        pass.set_vertex_buffers(0, &[(&self.vertex_buffer, 0), (&self.instance_buffer, 0)]);
        pass.set_index_buffer(&self.index_buffer, 0);
        pass.draw_indexed(
            0..self.indices.len() as u32,
            0,
            0..self.instance_count as u32,
        );
    }

    pub fn update(&mut self, system: &System, instance: &[Instance]) -> wgpu::CommandEncoder {
        let mut encoder = system
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        if instance.len() > 0 {
            self.instance_count = instance.len();
            let buffer_size = (instance.len() * std::mem::size_of::<Instance>()) as u64;
            let temp_buffer = system
                .device
                .create_buffer_mapped(instance.len(), wgpu::BufferUsage::COPY_SRC)
                .fill_from_slice(&instance);
            encoder.copy_buffer_to_buffer(&temp_buffer, 0, &self.instance_buffer, 0, buffer_size);
        }

        encoder
    }
}
