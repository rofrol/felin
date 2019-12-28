use crate::definitions::{Instance, Mesh, Vertex};


pub struct Batch {
    pub indices: Vec<u16>,
    pub vertices: Vec<Vertex>,
    pub instances: Vec<Instance>,
}

impl Batch {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            vertices: Vec::new(),
            instances: Vec::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh: &Mesh) {
        let new_indices: Vec<u16> = mesh
            .indices
            .clone()
            .iter_mut()
            .map(|indice| return (*indice as i64 + self.vertices.len() as i64) as u16)
            .collect();

        self.indices.extend(new_indices);
        self.vertices.extend(mesh.vertices.clone());
    }

    pub fn add_meshes(&mut self, mesh: Vec<&Mesh>) {
        mesh.iter().enumerate().for_each(|(index, item)| {
            if index > 0 {
                let last_vertice_len = mesh[index - 1].vertices.len();
                let new_indices: Vec<u16> = item
                    .indices
                    .clone()
                    .iter_mut()
                    .map(|indice| return (*indice as i64 + last_vertice_len as i64) as u16)
                    .collect();

                self.vertices.extend(item.vertices.clone());
                self.indices.extend(new_indices);
            } else {
                self.indices.extend(item.indices.clone());
                self.vertices.extend(item.vertices.clone());
            }
        });
    }

    pub fn add_instances(&mut self, instance: Instance) {
        self.instances.push(instance);
    }
}
