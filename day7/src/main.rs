use std::{fs, cell::RefCell};


type INodePtr = RefCell<Box<INode>>;

#[derive(Clone, Debug)]
enum Type {
    File{size: usize},
    Folder,
}

#[derive(Clone, Debug)]
struct INode {
    name: String,
    node_type: Type,
    children: Vec<INodePtr>,
}

impl INode {
    fn new(name: &str, n_type: Type) -> Self {
        Self {
            name: name.to_string(),
            node_type: n_type,
            children: vec![],
        }
    }

    fn add_child(&mut self, node: &INodePtr) {
        self.children.push(node.clone());
    }

    fn get_size(&self) -> usize {
        if let Type::File { size } = self.node_type {
            size
        } else if self.children.is_empty() {
            0
        } else {
            self.children.iter().map(|x| x.borrow().get_size()).sum::<usize>()
        }
    }

}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let mut f1 = INode::new("/", Type::Folder);
        let mut f2 = RefCell::new(Box::new(INode::new("game", Type::Folder)));

        f1.add_child(&mut f2);

        assert!(f1.children.len() != 0);
    }

    #[test]
    fn test_get_size() {
        let mut root = INode::new("/", Type::Folder);
        let mut game = RefCell::new(Box::new(INode::new("game", Type::Folder)));
        let mut music = RefCell::new(Box::new(INode::new("music", Type::Folder)));
        let csgo = RefCell::new(Box::new(INode::new("csgo", Type::File { size: 1500 })));
        let yellow = RefCell::new(Box::new(INode::new("yellow", Type::File { size: 300 })));

        game.borrow_mut().add_child(&csgo);
        music.borrow_mut().add_child(&yellow);
        root.add_child(&mut game);
        root.add_child(&mut music);


        assert!(root.get_size() == 1800);
        assert!(game.borrow().get_size() == 1500);
        assert!(csgo.borrow().get_size() == 1500);
        assert!(music.borrow().get_size() == 300);
    }
}