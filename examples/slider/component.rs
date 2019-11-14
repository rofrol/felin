use felin::definitions::Elements;
use felin::mesh::{Image, Rectangle};
use felin::utils::{Batch, Grid, Node, NodeWalker};
use felin::Event;

pub struct Slider {
    nodes: NodeWalker,
    pub batch: Batch,
}

impl Slider {
    pub fn new() {
            let mut tree = NodeWalker::create();
            let rect1 = Rectangle::new()
                .x(150.0)
                .y(150.0)
                .color([rand(0.0, 1.0), rand(0.0, 1.0), rand(0.0, 1.0), 1.0])
                .width(800.0)
                .height(700.0)
                .build();

            let parent = tree.add(Node {
                body: Elements::Rectangle(rect1.clone()),
                parent: None,
                grid: Some(Grid::new(rect1.width, rect1.height, "12/12")),
                area: None,
            });

            let rect = Rectangle::new()
                .color([rand(0.0, 1.0), rand(0.0, 1.0), rand(0.0, 1.0), 0.5])
                .build();

            tree.add(Node {
                body: Elements::Rectangle(rect.clone()),
                parent: Some(parent.clone()),
                grid: None,
                area: Some("0/3/0/3".to_string()),
            });

            tree.add(Node {
                body: Elements::Image(Image::new().use_texture(0).build()),
                parent: Some(parent.clone()),
                grid: None,
                area: Some("6/12/6/12".to_string()),
            });

            tree.add(Node {
                body: Elements::Image(Image::new().use_texture(0).build()),
                parent: Some(parent.clone()),
                grid: None,
                area: Some("0/6/6/12".to_string()),
            });

            Menu {
                batch: tree.get_batch(),
                nodes: tree,
            }
        }
    }
}