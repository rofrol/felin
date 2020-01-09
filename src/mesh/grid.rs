use crate::definitions::{Mesh, Vertex};
use crate::prelude::*;
use crate::utils::{Style};

#[derive(Clone)]
pub struct Grid<T: ElementCore> {
    pub style: Style,
    pub children: Vec<T>,
}

impl<T: ElementCore> Grid<T> {
    fn calculate_style(parent_style: Style, children: &mut Vec<T>) {
        for child in children.iter_mut() {
            let style = Style::calculate_style(parent_style, child.get_style());

            child.set_style(style);
            child.build();
        }
    }
}

#[allow(dead_code)]
impl<T: ElementCore> ElementCore for Grid<T> {
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
        Self::calculate_style(self.style.clone(), &mut self.children);
    }
    fn mesh(&self) -> Mesh<Vertex> {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}
