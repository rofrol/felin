use crate::definitions::{Mesh, Vertex};
use crate::utils::Style;

use lyon::math::*;
use lyon::tessellation;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::{BuffersBuilder, VertexBuffers};
use lyon::tessellation::FillOptions;

use cgmath::{self, prelude::*};
use collision::{prelude::*, primitive, Aabb2};

use crate::prelude::*;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Circle {
    pub style: Style,
    pub collider: Aabb2<f32>,
    pub color: [f32; 4],
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub id: Option<String>,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            style: Style::default(),
            collider: Aabb2 {
                min: cgmath::Point2::new(0.0, 0.0),
                max: cgmath::Point2::new(0.0, 0.0),
            },
            color: [1.0, 1.0, 1.0, 1.0],
            vertices: Vec::new(),
            indices: Vec::new(),
            id: None,
        }
    }
}

#[allow(dead_code)]
impl ElementCore for Circle {
    type Vertex = Vertex;
    fn build(&mut self) {
        let mut mesh: VertexBuffers<Vertex, u16> = VertexBuffers::new();
        let fill_options = FillOptions::tolerance(0.01);
        //Draw vertices with Lyon
        fill_circle(
            point(self.style.x, self.style.x),
            self.style.radius,
            &fill_options,
            &mut BuffersBuilder::new(&mut mesh, |vertex: tessellation::FillVertex| Vertex {
                in_position: vertex.position.to_array(),
                in_color: self.color,
                tex_pos: [0.0, 0.0],
                texture_id: -1,
            }),
        )
        .unwrap();

        self.collider = self.get_collider();
        self.vertices = mesh.vertices;
        self.indices = mesh.indices;
    }

    fn get_style(&self) -> Style {
        self.style
    }

    fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn mesh(&self) -> Mesh<Vertex> {
        Mesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }
}

impl ElementCollider for Circle {
    fn contains(&self, point: cgmath::Point2<f32>) -> bool {
        self.collider.contains(&point)
    }

    fn get_collider(&self) -> Aabb2<f32> {
        let transform: cgmath::Decomposed<cgmath::Vector2<f32>, cgmath::Basis2<f32>> =
            cgmath::Decomposed {
                scale: 1.0,
                rot: Rotation2::from_angle(cgmath::Rad(0.0)),
                disp: cgmath::Vector2::new(self.style.x, self.style.y),
            };
        return primitive::Circle::new(self.style.radius)
            .compute_bound()
            .transform(&transform);
    }
}
