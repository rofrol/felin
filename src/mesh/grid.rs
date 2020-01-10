use crate::definitions::{Mesh, Vertex};
use crate::prelude::*;
use crate::utils::{Batch, Style};

pub struct Grid<'a, 'b> {
    pub style: Style,
    pub children: &'a mut Vec<&'b mut dyn ElementCore<Vertex = Vertex>>,
}

#[allow(dead_code)]
impl<'a, 'b> ElementCore for Grid<'a, 'b> {
    type Vertex = Vertex;

    fn get_style(&self) -> Style {
        self.style
    }

    fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    fn get_id(&self) -> Option<String> {
        None
    }

    fn build(&mut self) {
        for child in self.children.iter_mut() {
            let style = Style::calculate_style(self.style, child.get_style());

            child.set_style(style);
            child.build();
        }
    }

    fn mesh(&self) -> Mesh<Vertex> {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}

impl<'a, 'b> Grid<'a, 'b> {
    pub fn finish(&mut self) -> &mut Self {
        for child in self.children.iter_mut() {
            let style = Style::calculate_style(self.style, child.get_style());

            child.set_style(style);
            child.build();
        }

        self
    }

    pub fn batch(&mut self, batches: &mut Vec<(String, &mut Batch<Vertex>)>) {
        for child in self.children.iter_mut() {
            match child.get_id() {
                Some(id) => {
                    for batch in batches.into_iter() {
                        if id.contains(&batch.0) {
                            batch.1.add(&mut child.mesh());
                        }
                    }
                }
                None => batches[0].1.add(&mut child.mesh()),
            }
        }
    }
}
