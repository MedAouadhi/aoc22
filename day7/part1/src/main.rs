use std::{cell::RefCell, fs, rc::Rc};

type Shared<T> = Rc<RefCell<T>>;
type INodePtr = Shared<INode>;
type INodeVec = Shared<Vec<INodePtr>>;

#[derive(Clone, Debug)]
enum Type {
    File { size: usize },
    Folder,
}

#[derive(Clone, Debug)]
struct INode {
    name: String,
    node_type: Type,
    parent: Option<INodePtr>,
    children: INodeVec,
}

impl INode {
    fn new(name: &str, n_type: Type) -> Self {
        Self {
            name: name.to_string(),
            node_type: n_type,
            parent: None,
            children: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn add_child(&mut self, node: INodePtr, parent: INodePtr) {
        node.borrow_mut().parent = Some(parent);
        self.children.borrow_mut().push(node)
    }

    fn get_size(&self, list: Option<Shared<Vec<usize>>>) -> usize {
        let sz = if let Type::File { size } = self.node_type {
            size
        } else if self.children.borrow_mut().is_empty() {
            0
        } else {
            self.children
                .borrow_mut()
                .iter()
                .map(|x| x.borrow().get_size(list.clone()))
                .sum::<usize>()
        };
        // save the size of the folders that are at most 100k
        if let Some(size_list) = list.clone() {
            if let Type::Folder = &self.node_type {
                if sz < 100000 {
                    size_list.borrow_mut().push(sz);
                }
            }
        }
        sz
    }

    fn get_child(&self, name: &str) -> Option<INodePtr> {
        self.children
            .borrow()
            .iter()
            .find(|&x| x.borrow().name == name)
            .cloned()
    }

    fn get_parent(&self) -> Option<INodePtr> {
        self.parent.clone()
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
}

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let commands: Vec<&str> = binding.split('$').collect();
    let root = INode::new("/", Type::Folder);

    let mut current: INodePtr = Rc::new(RefCell::new(root));
    let root_saved = current.clone();

    for &command in &commands[2..] {
        // command
        let mut args: Vec<&str> = command.split("\n").collect();
        args.retain(|s| !s.is_empty());

        let mut cmd_line: Vec<&str> = args[0].split(" ").collect();
        cmd_line.retain(|s| !s.is_empty());

        let cmd = cmd_line[0];

        match cmd.trim_start() {
            "cd" => {
                // we move through the inode
                let path = cmd_line[1];
                match path {
                    ".." => {
                        let parent_path = current.borrow().get_parent().unwrap();
                        current = parent_path;
                    }
                    &_ => {
                        let target = current.borrow().get_child(path).unwrap();
                        current = target;
                    }
                }
            }
            "ls" => {
                // we go through the inode
                for listing in &args[1..] {
                    let metadata: Vec<&str> = listing.split(" ").collect();
                    let name = metadata[1];
                    match metadata[0] {
                        "dir" => {
                            current.borrow_mut().add_child(
                                Rc::new(RefCell::new(INode::new(name, Type::Folder))),
                                current.clone(),
                            );
                        }
                        &_ => {
                            let size = metadata[0].parse::<usize>().unwrap();
                            current.borrow_mut().add_child(
                                Rc::new(RefCell::new(INode::new(name, Type::File { size }))),
                                current.clone(),
                            );
                        }
                    }
                }
            }
            &_ => unreachable!(),
        }
    }

    root_saved.borrow().print_contents(0);
    let sizes: Shared<Vec<usize>> = Rc::new(RefCell::new(Vec::new()));
    println!(
        "Total size of root = {}",
        root_saved.borrow().get_size(Some(sizes.clone()))
    );
    println!(
        "Sum of folders at most 100k = {}",
        sizes.borrow().iter().sum::<usize>()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_size() {
        // let mut root = INode::new("/", Type::Folder);
        let game = Rc::new(RefCell::new(INode::new("game", Type::Folder)));
        let music = Rc::new(RefCell::new(INode::new("music", Type::Folder)));
        let csgo = Rc::new(RefCell::new(INode::new("csgo", Type::File { size: 1500 })));
        let cod = Rc::new(RefCell::new(INode::new("cod", Type::File { size: 2000 })));
        let yellow = Rc::new(RefCell::new(INode::new("yellow", Type::File { size: 300 })));

        let root = Rc::new(RefCell::new(INode::new("/", Type::Folder)));
        root.borrow_mut().add_child(game.clone(), root.clone());
        root.borrow_mut().add_child(music.clone(), root.clone());

        game.borrow_mut().add_child(csgo.clone(), game.clone());
        game.borrow_mut().add_child(cod.clone(), game.clone());
        music.borrow_mut().add_child(yellow.clone(), music.clone());

        root.borrow().print_contents(0);

        assert!(root.borrow().get_size(None) == 3800);
        assert!(game.borrow().get_size(None) == 3500);
        assert!(csgo.borrow().get_size(None) == 1500);
        assert!(music.borrow().get_size(None) == 300);
    }

    #[test]
    fn test_get_child() {
        let root = Rc::new(RefCell::new(INode::new("/", Type::Folder)));
        let game = Rc::new(RefCell::new(INode::new("game", Type::Folder)));
        let music = Rc::new(RefCell::new(INode::new("music", Type::Folder)));
        let csgo = Rc::new(RefCell::new(INode::new("csgo", Type::File { size: 1500 })));
        let cod = Rc::new(RefCell::new(INode::new("cod", Type::File { size: 2000 })));
        let yellow = Rc::new(RefCell::new(INode::new("yellow", Type::File { size: 300 })));

        root.borrow_mut().add_child(game.clone(), root.clone());
        root.borrow_mut().add_child(music.clone(), root.clone());

        game.borrow_mut().add_child(csgo.clone(), game.clone());
        game.borrow_mut().add_child(cod.clone(), game.clone());
        music.borrow_mut().add_child(yellow.clone(), music.clone());

        root.borrow().print_contents(0);

        let child = root.borrow().get_child("music");
        // println!("child = {:#?}", child);
        assert!(child.is_some());
        child.unwrap().borrow().print_contents(0);
    }
}
