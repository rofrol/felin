use felin::mesh::{Grid, Image};
use felin::utils::Batch;
use felin::utils::Style;
use felin::Event;

#[allow(dead_code)]
pub struct Element {
    pub container: Batch,
    pub images: Batch,
}

impl Element {
    pub fn new() -> Self {
        let element = Grid {
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
                &mut Image {
                    style: Style {
                        row_start: 6,
                        row_end: 7,
                        column_start: 1,
                        column_end: 2,
                        ..Style::default()
                    },
                    texture: 1,
                    ..Default::default()
                },
                &mut Image {
                    style: Style {
                        row_start: 6,
                        row_end: 7,
                        column_start: 11,
                        column_end: 12,
                        ..Style::default()
                    },
                    texture: 3,
                    ..Default::default()
                },
                &mut Image {
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
            ..Default::default()
        }
        .into_batches(Some(vec!["slide".to_string()]));

        Element {
            container: element.get("default").unwrap().clone(),
            images: element.get("slide").unwrap().clone(),
        }
    }
}
