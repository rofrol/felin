mod rectangle;
pub use rectangle::Rectangle;


pub trait Element {
    fn render(&self);
}


pub struct Widget {
    pub name: String,
    pub body: Box<dyn Element>,
}

