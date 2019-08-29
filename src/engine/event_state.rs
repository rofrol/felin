use cgmath::{Point2};
use std::collections::{HashSet};
use wgpu::winit::MouseButton as Button;

#[derive(Debug)]
pub struct Mouse {
    pub position: Point2<u32>,
    pressed_buttons: HashSet<Button>,
}


impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            position: Point2::new(0,0),
            pressed_buttons: HashSet::new(),
        }
    }

    pub fn on_right_click(&self) -> bool {
        self.pressed_buttons.contains(&Button::Right)
    }

    pub fn on_left_right(&self) -> bool {
        self.pressed_buttons.contains(&Button::Left)
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.position = Point2::new(x, y);
    }

    pub fn button_pressed(&mut self, button: Button) {
        self.pressed_buttons.insert(button);
    }

    pub fn button_released(&mut self, button: Button) {
        self.pressed_buttons.remove(&button);
    }
}

#[derive(Debug)]
pub struct Event {
    pub mouse: Mouse,
}

impl Event {
    pub fn new() -> Event {
        Event {
            mouse: Mouse::new(),
        }
    }
}