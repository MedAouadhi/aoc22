use std::{cell::RefCell, fs, rc::Rc};

type INodePtr = Rc<RefCell<Box<INode>>>;

#[derive(Clone, Debug)]
enum Type {
    File { size: usize },
    Folder,
}

#[derive(Clone, Debug)]
struct INode {
    name: String,
    node_type: Type,
    children: RefCell<Vec<INodePtr>>,
}

impl INode {
    fn new(name: &str, n_type: Type) -> Self {
        Self {
            name: name.to_string(),
            node_type: n_type,
            children: RefCell::new(Vec::new()),
        }
    }

    fn add_child(&mut self, node: INodePtr) {
        self.children.borrow_mut().push(node)
    }

    fn get_size(&self) -> usize {
        if let Type::File { size } = self.node_type {
            size
        } else if self.children.borrow_mut().is_empty() {
            0
        } else {
            self.children
                .borrow_mut()
                .iter()
                .map(|x| x.borrow().get_size())
                .sum::<usize>()
        }
    }

    fn get_child(&self, name: &str) -> Option<INodePtr> {
        self.children
            .borrow()
            .iter()
            .find(|&x| x.borrow().name == name)
            .cloned()
    }

    fn print_contents(&self, depth: usize) {
        // print the current Node
        let tabs = "\t".repeat(depth);
        if let Type::File { size } = self.node_type {
            println!("{}{} {}", tabs, size, self.name);
        } else {
            println!("{}{}", tabs, self.name);
            return self
                .children
                .borrow_mut()
                .iter()
                .map(|x| x.borrow().print_contents(depth + 1))
                .collect();
        }
    }
    // fn from_path(&self, path: String) -> Option<&Self> {
    // }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_size() {
        let mut root = INode::new("/", Type::Folder);
        let mut game = Rc::new(RefCell::new(Box::new(INode::new("game", Type::Folder))));
        let mut music = Rc::new(RefCell::new(Box::new(INode::new("music", Type::Folder))));
        let csgo = Rc::new(RefCell::new(Box::new(INode::new(
            "csgo",
            Type::File { size: 1500 },
        ))));
        let cod = Rc::new(RefCell::new(Box::new(INode::new(
            "cod",
            Type::File { size: 2000 },
        ))));
        let yellow = Rc::new(RefCell::new(Box::new(INode::new(
            "yellow",
            Type::File { size: 300 },
        ))));

        root.add_child(game.clone());
        root.add_child(music.clone());

        game.borrow_mut().add_child(csgo.clone());
        game.borrow_mut().add_child(cod.clone());
        music.borrow_mut().add_child(yellow.clone());

        root.print_contents(0);

        assert!(root.get_size() == 3800);
        assert!(game.borrow().get_size() == 3500);
        assert!(csgo.borrow().get_size() == 1500);
        assert!(music.borrow().get_size() == 300);
    }

    #[test]
    fn test_get_child() {
        let mut root = INode::new("/", Type::Folder);
        let mut game = Rc::new(RefCell::new(Box::new(INode::new("game", Type::Folder))));
        let mut music = Rc::new(RefCell::new(Box::new(INode::new("music", Type::Folder))));
        let csgo = Rc::new(RefCell::new(Box::new(INode::new(
            "csgo",
            Type::File { size: 1500 },
        ))));
        let cod = Rc::new(RefCell::new(Box::new(INode::new(
            "cod",
            Type::File { size: 2000 },
        ))));
        let yellow = Rc::new(RefCell::new(Box::new(INode::new(
            "yellow",
            Type::File { size: 300 },
        ))));

        root.add_child(game.clone());
        root.add_child(music.clone());

        game.borrow_mut().add_child(csgo.clone());
        game.borrow_mut().add_child(cod.clone());
        music.borrow_mut().add_child(yellow.clone());

        let child = root.get_child("music");
        println!("child = {:#?}", child);
        assert!(child.is_some());
    }
}
