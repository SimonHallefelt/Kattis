use std::io;

fn main() {
    let mut root = Node::new(1);
    root.add_child(2);
    root.add_child(3);
    root.get_child_mut(0).add_child(4);
    root.get_child_mut(0).add_child(5);
    root.get_child_mut(1).add_child(6);
    root.get_child_mut(1).add_child(7);


    println!("Hello, world!");
}

pub struct Node {
    children: Vec<Node>,
    value: i64,
}

impl Node {
    pub fn new(d: i64) -> Node {
        Node {
            children: vec!(),
            value: d,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn add_child(&mut self, child_value: i64) {
        self.children.push(Node::new(child_value));
    }

    pub fn up_data(&mut self, value: i64) {
        self.value = value;
    }

    pub fn get_data(&self) -> i64 {
        self.value
    }

    pub fn get_children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn get_child(&self, index: usize) -> &Node {
        &self.children[index]
    }

    pub fn get_child_mut(&mut self, index: usize) -> &mut Node {
        &mut self.children[index]
    }
}
    
fn LowestCommonAncestor() {


}