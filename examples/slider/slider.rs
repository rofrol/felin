use felin::definitions::{Elements, Mesh};
use felin::mesh::{Image, Rectangle};
use felin::utils::{Batch, Grid, Node, NodeWalker};
use felin::Event;

use winit::window::WindowBuilder;

#[allow(dead_code)]
pub struct Slider {
    nodes: NodeWalker,
    slide: i32,
    width: f32,
    height: f32,
    pub container: Batch,
    pub gallery: Mesh,
}

#[allow(dead_code)]
impl Slider {
    pub fn update(&mut self, event: &Event) {
        self.build();
    }

    pub fn build(&mut self) {
        let container_rect = Rectangle::new()
            .x((self.width as usize / 2) as f32)
            .y(200.0)
            .color([1.0, 1.0, 1.0, 1.0])
            .width(self.width)
            .height(self.height)
            .build();

        let container = self.nodes.add(Node {
            grid: Some(Grid::new(
                container_rect.width,
                container_rect.height,
                "12/12",
            )),
            body: Elements::Rectangle(container_rect),
            parent: None,
            area: None,
        });

        self.nodes.add(Node {
            grid: None,
            body: Elements::Image(Image::new().use_texture(0).build()),
            parent: Some(container.clone()),
            area: Some("0/1/5/6".to_string()),
        });

        self.nodes.add(Node {
            grid: None,
            body: Elements::Image(Image::new().use_texture(1).build()),
            parent: Some(container.clone()),
            area: Some("11/12/5/6".to_string()),
        });

        let gallery = self.nodes.add(Node {
            grid: None,
            body: Elements::Image(Image::new().use_texture(self.slide).build()),
            parent: Some(container.clone()),
            area: Some("2/10/1/10".to_string()),
        });

        self.container = self.nodes.get_batch();
        self.gallery = self.nodes.get(&gallery).mesh();
    }

    pub fn new(window: &WindowBuilder) -> Self {
        let mut tree = NodeWalker::create();
        let window_params = window.window.inner_size.unwrap();

        Self {
            container: tree.get_batch(),
            nodes: tree,
            width: window_params.width as f32,
            height: window_params.height as f32,
            gallery: Image::new().use_texture(0).build().mesh(),
            slide: 1,
        }
    }
}
