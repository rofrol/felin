use cgmath::Point2;
use std::collections::HashSet;
use winit::event::MouseButton as Button;
use winit::event::{ElementState, MouseScrollDelta, WindowEvent};

#[derive(Debug)]
pub struct Mouse {
    pub position: Point2<f32>,
    pressed_buttons: HashSet<Button>,
    scroll_diff: f32,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            position: Point2::new(0.0, 0.0),
            pressed_buttons: HashSet::new(),
            scroll_diff: 0.0,
        }
    }

    pub fn on_right_click(&self) -> bool {
        self.pressed_buttons.contains(&Button::Right)
    }

    pub fn on_left_right(&self) -> bool {
        self.pressed_buttons.contains(&Button::Left)
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.position = Point2::new(x as f32, y as f32);
    }

    pub fn button_pressed(&mut self, button: Button) {
        self.pressed_buttons.insert(button);
    }

    pub fn button_released(&mut self, button: Button) {
        self.pressed_buttons.remove(&button);
    }

    pub fn x(&self) -> f32 {
        self.position[0]
    }

    pub fn y(&self) -> f32 {
        self.position[1]
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Keyboard {
    pub keys_pressed: HashSet<usize>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys_pressed: HashSet::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Event {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub dpi_factor: f64,
    pub resized: bool,
}

impl Event {
    pub fn new() -> Event {
        Event {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
            dpi_factor: 1.0,
            resized: false,
        }
    }

    pub fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            self.keyboard.keys_pressed.insert(keycode as usize);
                            // if let VirtualKeyCode::Back = keycode {
                            //     self.text.push(TextChar::Back);
                            // }
                        }
                        ElementState::Released => {
                            self.keyboard.keys_pressed.remove(&(keycode as usize));
                        }
                    }
                }
            }
            WindowEvent::ReceivedCharacter(_c) => {
                // if c != '\x08' && c != '\r' && c != '\n' {
                //     self.text.push(TextChar::Char(c));
                // }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position = position.to_physical(self.dpi_factor);
                self.mouse
                    .set_position(position.x as u32, position.y as u32)
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                self.mouse.button_pressed(button);
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                self.mouse.button_released(button);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                const PIXELS_PER_LINE: f64 = 38.0;

                match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        self.mouse.scroll_diff += y;
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        self.mouse.scroll_diff += (delta.y / PIXELS_PER_LINE) as f32
                    }
                }
            }
            _ => {}
        }
    }

    pub fn clear(&mut self) {
        self.resized = false;
    }
}
