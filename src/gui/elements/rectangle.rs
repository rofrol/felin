use crate::gui::elements::Element;
use wgpu;


#[derive(Debug)]
pub struct Rectangle {
    pub name: String,
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            name: String::from("rectangle"),
        }
    }
}

impl Element for Rectangle { 
    fn render(&self) {
        println!("{}", self.name);
    }
}