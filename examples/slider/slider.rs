use felin::mesh::{Grid, Image, Rectangle};
use felin::prelude::*;
use felin::utils::Batch;
use felin::utils::Style;
use felin::Event;

#[allow(dead_code)]
pub struct Element {
    pub elements: Batch,
}

impl Element {
    pub fn new() -> Self {
        let element = Grid {
            style: Style {
                width: 600.0,
                height: 600.0,
                x: 200.0,
                y: 200.0,
                rows: 12,
                columns: 12,
                ..Style::default()
            },
            children: vec![
                &mut Rectangle {
                    style: Style {
                        row_start: 1,
                        row_end: 12,
                        column_start: 1,
                        column_end: 5,
                        ..Style::default()
                    },
                    color: [1.0, 1.0, 1.0, 1.0],
                    ..Default::default()
                },
                &mut Rectangle {
                    style: Style {
                        row_start: 1,
                        row_end: 12,
                        column_start: 6,
                        column_end: 12,
                        ..Style::default()
                    },
                    color: [1.0, 1.0, 1.0, 1.0],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
        .into_batches(None);

        Element {
            elements: element.get("default").unwrap().clone(),
        }
    }
}
