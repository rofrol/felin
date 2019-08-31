# wgpu-2d

This is an exercise repo for myself to learn how to handle vulkan and 2D graphics, it use's `wgpu-rs` as graphics backend.


To try it, include master branch on your cargo

```rust
[dependencies.engine]
{ git = "https://github.com/tonis2/wgpu-2d", branch = "master" }

```


```rust
#![feature(box_syntax)]


use engine::{Base, App, RenderPass, Window, Event };
use engine::prelude::*;
use engine::gui::{
    Widget,
    elements::{Triangle, Circle},
};


pub struct Main {
    circle: Circle,
}

impl Base for Main {
    fn init(_window: &mut Window) -> Self {


    let circle = Circle::new()
        .x(300.0)
        .y(300.0)
        .radius(20.0)
        .build();
        
        Main {
            circle,
        }
    }


    fn update(&mut self, event: &Event) {
        if event.mouse.on_left_right() {
            self.circle.x(event.mouse.x());
            self.circle.y(event.mouse.y());
        }
    }
    
    fn render(&mut self, _window: &mut Window, rpass: &mut RenderPass) {    
        self.circle.render(rpass);
    }
}

fn main() {
    App::new().init::<Main>("Title")
}



```
