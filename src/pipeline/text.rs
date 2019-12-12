use crate::definitions::Vertex;
use crate::utils::{load_glsl, FontPallet, ShaderStage};
use crate::System;
use cgmath::{self, prelude::*};

#[allow(dead_code)]
pub struct Pipeline {
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    texture_layout: wgpu::BindGroupLayout,
    render_pipeline: wgpu::RenderPipeline,
}

#[allow(dead_code)]
impl Pipeline {
    pub fn new(system: &mut System) -> Self {
        let bind_group_layout =
            system
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let texture_layout =
            system
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[
                        wgpu::BindGroupLayoutBinding {
                            binding: 0,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D2,
                            },
                        },
                        wgpu::BindGroupLayoutBinding {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler,
                        },
                    ],
                });

        let pipeline_layout =
            system
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&bind_group_layout, &texture_layout],
                });

        let default_transform: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
        let transform_buf: &[f32; 16] = default_transform.as_ref();

        let matrix = system.get_screen_matrix();
        let ortho_buffer: &[f32; 16] = matrix.as_ref();

        let uniform_buffer = system
            .device
            .create_buffer_mapped(16, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
            .fill_from_slice(ortho_buffer);

        let transform_buffer = system
            .device
            .create_buffer_mapped(16, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
            .fill_from_slice(transform_buf);

        let bind_group = system.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        range: 0..64,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &transform_buffer,
                        range: 0..64,
                    },
                },
            ],
        });

        let vs_bytes = load_glsl(include_str!("shaders/text.vert"), ShaderStage::Vertex);
        let fs_bytes = load_glsl(include_str!("shaders/text.frag"), ShaderStage::Fragment);
        let vs_module = system.device.create_shader_module(&vs_bytes);
        let fs_module = system.device.create_shader_module(&fs_bytes);

        let render_pipeline =
            system
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
                    primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                    color_states: &[wgpu::ColorStateDescriptor {
                        format: wgpu::TextureFormat::Bgra8Unorm,
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
                                offset: 8,
                                shader_location: 1,
                            },
                            wgpu::VertexAttributeDescriptor {
                                format: wgpu::VertexFormat::Float2,
                                offset: 24,
                                shader_location: 2,
                            },
                            wgpu::VertexAttributeDescriptor {
                                format: wgpu::VertexFormat::Int,
                                offset: 32,
                                shader_location: 3,
                            },
                        ],
                    }],
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                });

        Self {
            uniform_buffer,
            bind_group,
            texture_layout,
            render_pipeline,
        }
    }

    pub fn create_font_texture(
        &mut self,
        system: &mut System,
        font_instance: &FontPallet,
    ) -> wgpu::BindGroup {
        let sampler = system.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare_function: wgpu::CompareFunction::Always,
        });

        let texture_extent = wgpu::Extent3d {
            width: font_instance.max_w as u32,
            height: font_instance.max_h as u32,
            depth: 1,
        };

        let texture = system.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
        });

        let mut encoder = system
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        for (_key, value) in font_instance.characters.iter() {
            let image_buffer = system
                .device
                .create_buffer_mapped(value.data.len(), wgpu::BufferUsage::COPY_SRC)
                .fill_from_slice(&value.data);

            encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &image_buffer,
                    offset: 0,
                    row_pitch: value.width as u32,
                    image_height: value.height as u32,
                },
                wgpu::TextureCopyView {
                    texture: &texture,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d {
                        x: value.x as f32,
                        y: value.y as f32,
                        z: 0.0,
                    },
                },
                wgpu::Extent3d {
                    width: value.width,
                    height: value.height,
                    depth: 1,
                },
            );
        }

        let texture_view = texture.create_default_view();

        let bind_group = system.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        system.queue.submit(&[encoder.finish()]);

        bind_group
    }

    pub fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        system: &System,
        indices: &Vec<u16>,
        vertices: &Vec<Vertex>,
        textures: &wgpu::BindGroup,
    ) {
        let vertex_buffer = system
            .device
            .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&vertices);

        let index_buffer = system
            .device
            .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&indices);

        pass.set_pipeline(&self.render_pipeline);

        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_bind_group(1, textures, &[]);

        pass.set_index_buffer(&index_buffer, 0);
        pass.set_vertex_buffers(0, &[(&vertex_buffer, 0)]);
        pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }

    pub fn resize(&mut self, system: &mut System) {
        let screen_matrix = system.get_screen_matrix();
        let mx_ref: &[f32; 16] = screen_matrix.as_ref();

        let temp_buf = system
            .device
            .create_buffer_mapped(16, wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(mx_ref);

        let mut encoder = system
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        encoder.copy_buffer_to_buffer(&temp_buf, 0, &self.uniform_buffer, 0, 64);

        system.queue.submit(&[encoder.finish()]);
    }
}
