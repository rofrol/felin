use felin::definitions::Vertex;
use felin::mesh::{Grid, Image};
use felin::prelude::*;
use felin::utils::{Batch, Event, Style};

#[allow(dead_code)]
pub struct Element {
    pub container: Batch<Vertex>,
    pub images: Batch<Vertex>,

    left_button: Image,
    right_button: Image,
    slider: Image,
    max_slides: i32,
}

impl Element {
    pub fn update(&mut self, event: &Event) {
        if self.left_button.contains(event.mouse.position) {
            self.left_button.texture = 1;
        } else {
            self.left_button.texture = 0;
        }

        if self.right_button.contains(event.mouse.position) {
            self.right_button.texture = 3;
        } else {
            self.right_button.texture = 2;
        }

        if self.left_button.contains(event.mouse.position) && event.mouse.on_left_click() {
            if self.slider.texture != 0 {
                self.slider.texture -= 1;
            }
        }

        if self.right_button.contains(event.mouse.position) && event.mouse.on_left_click() {
            if self.slider.texture < self.max_slides - 1 {
                self.slider.texture += 1;
            }
        }

        self.render();
    }

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
                &mut self.left_button,
                &mut self.right_button,
                &mut self.slider,
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

    pub fn new(max_slides: i32) -> Self {
        let left_button = Image {
            style: Style {
                row_start: 6,
                row_end: 7,
                column_start: 1,
                column_end: 2,
                ..Style::default()
            },
            texture: 0,
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

        let slider = Image {
            style: Style {
                row_start: 2,
                row_end: 11,
                column_start: 2,
                column_end: 11,
                ..Style::default()
            },
            id: Some("slide".to_string()),
            texture: 0,
            color: [1.0, 1.0, 1.0, 1.0],
            ..Default::default()
        };

        Element {
            container: Batch::new(),
            images: Batch::new(),
            left_button,
            right_button,
            slider,
            max_slides,
        }
    }
}
