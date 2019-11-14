use super::batch::Batch;
use crate::definitions::Elements;
use crate::utils::Grid;

//Single Node

#[derive(Clone)]
pub struct Node {
    pub body: Elements,
    pub parent: Option<froggy::Pointer<Node>>,
    pub grid: Option<Grid>,
    pub area: Option<String>,
}

impl Node {
    pub fn get_grid(&mut self) -> Grid {
        self.grid.unwrap()
    }
}

//Node tree
pub struct NodeWalker {
    tree: froggy::Storage<Node>,
}

impl NodeWalker {
    pub fn create() -> NodeWalker {
        NodeWalker {
            tree: froggy::Storage::new(),
        }
    }

    pub fn get_tree(&mut self) -> &mut froggy::Storage<Node> {
        &mut self.tree
    }

    pub fn cursor(&mut self) -> froggy::Cursor<Node> {
        self.tree.cursor()
    }

    pub fn add(&mut self, node: Node) -> froggy::Pointer<Node> {
        if node.parent.is_some() {
            let parent = &mut self.tree[&node.parent.clone().unwrap()];
            let result = parent.get_grid().get_position(&node.area.clone().unwrap());
            let mut clone_node = node.clone();

            let x = match parent.body {
                Elements::Rectangle(ref mut element) => {
                    (element.x.clone() as usize + result.x as usize) as f32
                }
                Elements::Circle(ref mut element) => {
                    (element.x.clone() as usize + result.x as usize) as f32
                }
                Elements::Image(ref mut element) => {
                    (element.x.clone() as usize + result.x as usize) as f32
                }
            };

            let y = match parent.body {
                Elements::Rectangle(ref mut element) => {
                    (element.y.clone() as usize + result.y as usize) as f32
                }
                Elements::Circle(ref mut element) => {
                    (element.y.clone() as usize + result.y as usize) as f32
                }
                Elements::Image(ref mut element) => {
                    (element.y.clone() as usize + result.y as usize) as f32
                }
            };

            match clone_node.body {
                Elements::Rectangle(ref mut element) => {
                    if let Elements::Rectangle(el) = node.body {
                        clone_node.body = Elements::Rectangle(
                            element
                                .x(x)
                                .y(y)
                                .width(result.width)
                                .height(result.height)
                                .color(el.color)
                                .build(),
                        );
                    }
                }
                Elements::Circle(ref mut element) => {
                    clone_node.body = Elements::Circle(element.x(x).y(y).build());
                }
                Elements::Image(ref mut element) => {
                    clone_node.body = Elements::Image(
                        element
                            .x(x)
                            .y(y)
                            .width(result.width)
                            .height(result.height)
                            .build(),
                    );
                }
            };

            return self.tree.create(clone_node);
        } else {
            return self.tree.create(node);
        }
    }

    pub fn get_batch(&mut self) -> Batch {
        let mut batch = Batch::new();

        for node in self.tree.iter_mut() {
            match node.body {
                Elements::Rectangle(ref mut element) => batch.add_mesh(&element.mesh()),
                Elements::Circle(ref mut element) => batch.add_mesh(&element.mesh()),
                Elements::Image(ref mut element) => batch.add_mesh(&element.mesh()),
            }
        }

        batch
    }
}
