use crate::definitions::{Mesh, Vertex};
use crate::prelude::*;
use crate::utils::{Batch, Style};
use collision::{prelude::*, primitive, Aabb2};

pub struct Grid<'a> {
    pub style: Style,
    pub children: Vec<&'a mut dyn ElementCore>,
}

impl<'a> Default for Grid<'a> {
    fn default() -> Self {
        Self {
            style: Style::default(),
            children: Vec::new(),
        }
    }
}

impl<'a> Grid<'a> {
    fn calculate_style(&mut self) {}

    pub fn build(&mut self) {
        for child in self.children.iter_mut() {
            let style = Style::calculate_style(self.style, child.get_style());
            child.set_style(style);
        }
    }

    pub fn into_batch(&mut self) -> Batch {
        self.build();

        let mut batch = Batch::new();

        for child in self.children.iter_mut() {
            child.build();
            batch.add_mesh(&child.mesh());
        }

        batch
    }
}

#[allow(dead_code)]
impl<'a> ElementCore for Grid<'a> {
    fn build(&mut self) {}
    fn get_style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}
