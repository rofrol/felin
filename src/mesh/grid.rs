use crate::definitions::Mesh;
use crate::prelude::*;
use crate::utils::{Batch, Style};
use std::collections::HashMap;

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
        batch: &mut HashMap<String, Batch>,
    ) {
        for child in children.iter_mut() {
            let style = Style::calculate_style(parent_style, child.get_style());

            child.set_style(style);
            child.build();

            match child.get_id() {
                // The division was valid
                Some(id) => {
                    if batch.contains_key(&id) {
                        batch.get_mut(&id).unwrap().add_mesh(&child.mesh());
                    }
                }
                // The division was invalid
                None => batch.get_mut("default").unwrap().add_mesh(&child.mesh()),
            }
        }
    }

    pub fn into_batches(&mut self, filters: Option<Vec<String>>) -> HashMap<String, Batch> {
        let mut batch: HashMap<String, Batch> = [("default".to_string(), Batch::new())]
            .iter()
            .cloned()
            .collect();

        if filters.is_some() {
            for item in filters.as_ref().unwrap() {
                batch.insert(item.to_string(), Batch::new());
            }
        }

        Self::calculate_style(self.style.clone(), &mut self.children, &mut batch);
        batch
    }
}

#[allow(dead_code)]
impl<'a> ElementCore for Grid<'a> {
    fn get_style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    fn get_id(&self) -> Option<String> {
        None
    }

    fn build(&mut self) {}
    fn mesh(&mut self) -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}
