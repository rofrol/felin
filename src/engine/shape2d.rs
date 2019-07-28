use wgpu;
use cgmath::{Matrix4};
use crate::engine::{ShaderStage, load_glsl};
use crate::gui::definitions::Vertex;

///////////////////////////////////////////////////////////////////////////
// Uniforms
///////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uniforms {
    pub ortho: Matrix4<f32>,
    pub transform: Matrix4<f32>,
}


///////////////////////////////////////////////////////////////////////////
// Pipeline
///////////////////////////////////////////////////////////////////////////


pub struct Pipeline {
   pub bind_group: wgpu::BindGroup, 
   pub render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device) -> Pipeline {

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor { bindings: &[] }
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[],
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
        }
    }
}
