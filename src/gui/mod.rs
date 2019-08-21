pub mod definitions;
pub mod elements;
use froggy;

pub struct Registry {
    pub widget_registry: froggy::Storage<elements::Widget>,
}

impl Registry {
    pub fn new() -> Registry {
         Registry {
             widget_registry: froggy::Storage::new(),
         }
    }

    pub fn add(&mut self, widget: elements::Widget) {
        self.widget_registry.create(widget);
    }

    pub fn draw(&self) {
        for value in self.widget_registry.iter_all() {
        
            value.body.show_name();
        }
    }
}