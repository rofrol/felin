use felin::definitions::Elements;
use felin::mesh::{Image, Rectangle};
use felin::utils::{Batch, Grid, Node, NodeWalker};
use felin::Event;

pub struct Slider {
    nodes: NodeWalker,
    pub batch: Batch,
}

impl Slider {
    pub fn new() -> Self {
        let mut tree = NodeWalker::create();

        let container_rect = Rectangle::new()
            .x(200.0)
            .y(200.0)
            .color([1.0, 1.0, 1.0, 1.0])
            .width(800.0)
            .height(600.0)
            .build();

        let container = tree.add(Node {
            grid: Some(Grid::new(container_rect.width, container_rect.height, "12/12")),
            body: Elements::Rectangle(container_rect),
            parent: None,
            area: None,
        });

        tree.add(Node {
            grid: None,
            body: Elements::Image(Image::new().use_texture(0).build()),
            parent: Some(container.clone()),
            area: Some("1/4/5/8".to_string()),
        });


        tree.add(Node {
            grid: None,
            body: Elements::Image(Image::new().use_texture(1).build()),
            parent: Some(container.clone()),
            area: Some("8/12/5/8".to_string()),
        });

        Self {
            batch: tree.get_batch(),
            nodes: tree,
        }
    }
}
