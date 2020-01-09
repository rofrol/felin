use felin::definitions::Vertex;
use felin::mesh::{Grid, Image};
use felin::prelude::*;
use felin::utils::Batch;
use felin::utils::Style;
use felin::Event;

#[allow(dead_code)]
pub struct Element {
    pub container: Batch<Vertex>,
    pub images: Batch<Vertex>,

    left_button: Image,
    right_button: Image,
    current_slide: i32,
}

impl Element {
    pub fn render(&mut self) {
        self.container.clear();
        self.images.clear();
        let mut grid = Grid {
            style: Style {
                width: 1500.0,
                height: 1100.0,
                x: 500.0,
                y: 100.0,
                rows: 12,
                columns: 12,
                ..Style::default()
            },
            children: vec![
                self.left_button.clone(),
                self.right_button.clone(),
                Image {
                    style: Style {
                        row_start: 2,
                        row_end: 11,
                        column_start: 3,
                        column_end: 10,
                        ..Style::default()
                    },
                    id: Some("slide".to_string()),
                    texture: 0,
                    color: [1.0, 1.0, 1.0, 1.0],
                    ..Default::default()
                },
            ],
        };
        grid.build();

        //Add the elements to differenct batches
        for child in grid.children.iter_mut() {
            match child.get_id() {
                Some(id) => {
                    if id.contains("slide") {
                        self.images.add(&mut child.mesh());
                    }
                }
                None => self.container.add(&mut child.mesh()),
            }
        }
    }

    pub fn new() -> Self {
        let left_button = Image {
            style: Style {
                row_start: 6,
                row_end: 7,
                column_start: 1,
                column_end: 2,
                ..Style::default()
            },
            texture: 1,
            ..Default::default()
        };

        let right_button = Image {
            style: Style {
                row_start: 6,
                row_end: 7,
                column_start: 11,
                column_end: 12,
                ..Style::default()
            },
            texture: 3,
            ..Default::default()
        };

        Element {
            container: Batch::new(),
            images: Batch::new(),
            left_button,
            right_button,
            current_slide: 0,
        }
    }
}
