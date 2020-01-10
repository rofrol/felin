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
    //Todo can be made faster
    pub fn add(&mut self, mesh: &mut Mesh<T>) {
        let new_indices: Vec<u16> = mesh
            .get_indices()
            .clone()
            .iter_mut()
            .map(|indice| return (*indice as i64 + self.vertices.len() as i64) as u16)
            .collect();

        self.indices.extend(new_indices);
        self.vertices.extend(mesh.get_vertices());
    }
}
