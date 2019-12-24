use felin::definitions::{Elements, Mesh};
use felin::mesh::{Image, Rectangle};
use felin::utils::{Batch, Grid, Node, NodeWalker};
use felin::Event;

#[allow(dead_code)]
pub struct Slider {
    nodes: NodeWalker,
    slide: i32,
    pub container: Batch,
    pub gallery: Mesh,
}

#[allow(dead_code)]
impl Slider {
    pub fn update(&mut self, event: &Event) {
        for node in self.nodes.get_tree() {
            if node.id.contains("button") {
                match node.body {
                    Elements::Image(ref mut element) => {
                        if element.contains(event.mouse.position) {
                            if node.id.contains("button_left") {
                                element.use_texture(1);
                                node.body = Elements::Image(element.build());
                                if event.mouse.on_left_click() {
                                    if self.slide > 0 {
                                        self.slide -= 1;
                                    }
                                }
                            } else if node.id.contains("button_right") {
                                element.use_texture(3);
                                node.body = Elements::Image(element.build());
                                if event.mouse.on_left_click() {
                                    if self.slide < 2 {
                                        self.slide += 1;
                                    }
                                }
                            }
                        } else {
                            if node.id.contains("button_left") {
                                element.use_texture(0);
                                node.body = Elements::Image(element.build());
                            } else if node.id.contains("button_right") {
                                element.use_texture(2);
                                node.body = Elements::Image(element.build());
                            }
                        }
                    }
                    _ => {}
                };
            }

            if node.id == "slide".to_string() {
                if let Elements::Image(ref mut slider) = node.body {
                    self.gallery = slider.use_texture(self.slide).build().mesh();
                }
            }
        }

        self.container = self.nodes.get_batch();
    }
    pub fn new() -> Self {
        let mut tree = NodeWalker::create();

        let container_rect = Rectangle::new()
            .x((800 / 2) as f32)
            .y(200.0)
            .color([0.52, 0.73, 0.94, 1.0])
            .width(800 as f32)
            .height(800 as f32)
            .build();

        let container = tree.add(Node {
            grid: Some(Grid::new(
                container_rect.width,
                container_rect.height,
                "12/12",
            )),
            body: Elements::Rectangle(container_rect),
            parent: None,
            area: None,
            id: "container".to_string(),
        });

        tree.add(Node {
            grid: None,
            body: Elements::Image(Image::new().use_texture(0).build()),
            parent: Some(container.clone()),
            area: Some("0/1/5/6".to_string()),
            id: "button_left".to_string(),
        });

        tree.add(Node {
            grid: None,
            body: Elements::Image(Image::new().use_texture(2).build()),
            parent: Some(container.clone()),
            area: Some("11/12/5/6".to_string()),
            id: "button_right".to_string(),
        });

        let gallery = tree.add(Node {
            grid: None,
            body: Elements::Image(Image::new().use_texture(0).build()),
            parent: Some(container.clone()),
            area: Some("1/11/0/12".to_string()),
            id: "slide".to_string(),
        });

        Self {
            container: tree.get_batch(),
            gallery: tree.get(&gallery).mesh(),
            nodes: tree,
            slide: 0,
        }
    }
}
