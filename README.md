# wgpu-2d

This is an exercise repo for myself to learn how to handle vulkan and 2D graphics, it use's `wgpu-rs` as graphics backend.


To try it, include master branch on your cargo

```rust
[dependencies.engine]
{ git = "https://github.com/tonis2/wgpu-2d", branch = "master" }

```


```rust
use engine::{Base, App, Frame, Window, Event };
use engine::prelude::*;
use engine::gui::{
    Widget,
    elements::{Circle, Rectangle},
};


pub struct Main {
    circle: Circle,
    rectangle: Rectangle,
}

impl Base for Main {
    fn init(_window: &mut Window) -> Self {

    let circle = Circle::new()
        .x(300.0)
        .y(300.0)
        .radius(20.0)
        .build();

    let rectangle = Rectangle::new()
        .x(100.0)
        .y(100.0)
        .width(150.0)
        .height(80.0)
        .build();
        
        Main {
            circle,
            rectangle,
        }
    }

    fn update(&mut self, event: &Event) {
        if event.mouse.on_left_right() {
           if self.circle.contains(event.mouse.position) {
               println!("Clicked on circle");
               self.circle.x(self.circle.x + 5.0);
           }

           if self.rectangle.contains(event.mouse.position) {
               println!("Clicked on rectangle");
           }
        }
    }
    
    fn render(&mut self, _window: &mut Window, frame: &mut Frame) {
        frame.clear([0.2, 0.0, 0.0, 0.5]);

        frame.draw(self.circle.render());
        frame.draw(self.rectangle.render());  
    }
}

fn main() {
    App::<Main>("App")
}




```
