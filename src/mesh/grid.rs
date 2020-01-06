use crate::definitions::Mesh;
use crate::prelude::*;
use crate::utils::{Batch, Style};

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
    fn calculate_style(
        parent_style: Style,
        children: &mut Vec<&'a mut dyn ElementCore>,
        batch: &mut Batch,
    ) {
        for child in children.iter_mut() {
            let style = Style::calculate_style(parent_style, child.get_style());
            child.set_style(style);
            child.build();

            batch.add_mesh(&child.mesh());
        }
    }

    pub fn into_batch(&mut self) -> Batch {
        let mut batch = Batch::new();
        Self::calculate_style(self.style.clone(), &mut self.children, &mut batch);
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
