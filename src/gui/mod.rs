pub mod definitions;
pub mod elements;

pub use definitions::{Element, Widget};

use froggy;

pub struct Registry {
    pub entries: froggy::Storage<definitions::Widget>,
}

impl Registry {
    pub fn new() -> Registry {
         Registry {
             entries: froggy::Storage::new(),
         }
    }

    pub fn add(&mut self, widget: definitions::Widget) {
        self.entries.create(widget);
    }
}