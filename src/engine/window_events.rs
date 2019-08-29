use cgmath::Point2;
use std::collections::HashSet;
use winit::event::MouseButton as Button;
use winit::event::{self, ElementState, WindowEvent};

#[derive(Debug)]
pub struct Mouse {
    pub position: Point2<u32>,
    pressed_buttons: HashSet<Button>,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            position: Point2::new(0, 0),
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

    pub fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                // if let Some(keycode) = input.virtual_keycode {
                //     match input.state {
                //         ElementState::Pressed => {
                //             self.key_held[keycode as usize] = true;
                //             self.key_actions.push(KeyAction::Pressed(keycode));
                //             if let VirtualKeyCode::Back = keycode {
                //                 self.text.push(TextChar::Back);
                //             }
                //         }
                //         ElementState::Released => {
                //             self.key_held[keycode as usize] = false;
                //             self.key_actions.push(KeyAction::Released(keycode));
                //         }
                //     }
                // }
            }
            WindowEvent::ReceivedCharacter(c) => {
                // if c != '\x08' && c != '\r' && c != '\n' {
                //     self.text.push(TextChar::Char(c));
                // }
            }
            WindowEvent::CursorMoved { position, .. } => {
                // let position = position.to_physical(self.dpi_factor);
                // self.mouse_point = Some((position.x as f32, position.y as f32));
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                // let button = mouse_button_to_int(button);
                // self.mouse_held[button] = true;
                // self.mouse_actions.push(MouseAction::Pressed(button));
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                // let button = mouse_button_to_int(button);
                // self.mouse_held[button] = false;
                // self.mouse_actions.push(MouseAction::Released(button));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // I just took this from three-rs, no idea why this magic number was chosen ¯\_(ツ)_/¯
                // const PIXELS_PER_LINE: f64 = 38.0;

                //match delta {
                // MouseScrollDelta::LineDelta  (_, y) => { self.scroll_diff += y; }
                // MouseScrollDelta::PixelDelta (delta) => { self.scroll_diff += (delta.y / PIXELS_PER_LINE) as f32 }
                // }
            }
            WindowEvent::Resized(resolution) => {
                // self.resolution = resolution.to_physical(self.dpi_factor).into();
            }
            WindowEvent::HiDpiFactorChanged(factor) => {
                // self.dpi_factor = factor;
            }
            _ => {}
        }
    }
}
