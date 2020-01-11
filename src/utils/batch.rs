use crate::definitions::{Mesh, MeshTrait};

#[derive(Clone)]
pub struct Batch<T: Clone> {
    pub indices: Vec<u16>,
    pub vertices: Vec<T>,
}

impl<T: Clone> Batch<T> {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            vertices: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn add(&mut self, mesh: &mut Mesh<T>) {
        for indice in mesh.get_indices() {
            self.indices
                .push((indice as i64 + self.vertices.len() as i64) as u16);
        }

        self.vertices.extend(mesh.get_vertices());
    }
}
