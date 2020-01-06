use crate::definitions::{Mesh, Vertex};

use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;

use collision::{prelude::*, primitive, Aabb2};

use crate::utils::Style;

use crate::prelude::*;

#[derive(Clone)]
pub struct Rectangle {
    pub style: Style,
    pub color: [f32; 4],
    pub collider: Aabb2<f32>,
    pub buffers: VertexBuffers<Vertex, u16>,
    pub id: Option<String>,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            style: Style::default(),
            collider: Aabb2 {
                min: cgmath::Point2::new(0.0, 0.0),
                max: cgmath::Point2::new(0.0, 0.0),
            },
            buffers: VertexBuffers::new(),
            color: [1.0, 1.0, 1.0, 1.0],
            id: None,
        }
    }
}

impl Rectangle {
    pub fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.buffers.vertices.clone(),
            indices: self.buffers.indices.clone(),
        }
    }
    pub fn color(&mut self, color: [f32; 4]) {
        self.color = color;
    }
}

#[allow(dead_code)]
impl ElementCore for Rectangle {
    fn build(&mut self) {
        let mut buffers: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);

        //Draw vertices with Lyon
        fill_rectangle(
            &rect(
                self.style.x,
                self.style.y,
                self.style.width,
                self.style.height,
            ),
            &fill_options,
            &mut BuffersBuilder::new(&mut buffers, |vertex: tessellation::FillVertex| Vertex {
                in_position: vertex.position.to_array(),
                in_color: self.color,
                tex_pos: [0.0, 0.0],
                texture_id: -1,
            }),
        )
        .unwrap();

        self.buffers = buffers;
        self.collider = self.get_collider();
    }

    fn get_style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: self.buffers.vertices.clone(),
            indices: self.buffers.indices.clone(),
        }
    }

    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }
}

impl ElememtResizable for Rectangle {
    fn width(&mut self, width: f32) {
        self.style.width = width;
    }

    fn height(&mut self, height: f32) {
        self.style.height = height;
    }

    fn radius(&mut self, _radius: f32) {}
}

impl ElementCollider for Rectangle {
    fn contains(&self, point: cgmath::Point2<f32>) -> bool {
        self.collider.contains(&point)
    }

    fn get_collider(&self) -> Aabb2<f32> {
        let transform: cgmath::Decomposed<cgmath::Vector2<f32>, cgmath::Basis2<f32>> =
            cgmath::Decomposed {
                scale: 1.0,
                rot: cgmath::Rotation2::from_angle(cgmath::Rad(0.0)),
                disp: cgmath::Vector2::new(
                    self.style.x + (self.style.width / 2.0),
                    self.style.y + (self.style.height / 2.0),
                ),
            };
        primitive::Rectangle::new(self.style.width, self.style.height)
            .compute_bound()
            .transform(&transform)
    }
}
