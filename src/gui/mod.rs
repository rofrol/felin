pub mod elements;

use crate::definitions;
use froggy;

pub struct ElementRegistry {
    pub entries: froggy::Storage<definitions::Widget>,
}

impl ElementRegistry {
    pub fn new() -> ElementRegistry {
         ElementRegistry {
             entries: froggy::Storage::new(),
         }
    }

    pub fn define(&mut self, widget: definitions::Widget) {
        self.entries.create(widget);
    }
}