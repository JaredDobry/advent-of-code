use std::{cell::RefCell, fmt::Display, rc::Rc, vec::Vec};

struct Node {
    name: String,
    size: i32,
    parent: Option<Rc<Node>>,
    children: Option<RefCell<Vec<Rc<Node>>>>,
}

impl Node {
    // Consumes the Rc
    fn add_child(&self, child: Rc<Node>) {
        match &self.children {
            Some(c) => c.borrow_mut().push(child),
            None => return,
        }
    }

    fn child_names(&self) -> String {
        let mut child_names = String::from("[");
        let mut first = true;
        match &self.children {
            Some(c) => {
                for child in c.borrow().iter() {
                    if first {
                        first = false;
                    } else {
                        child_names.push_str(", ")
                    }
                    child_names.push_str(child.name.as_str())
                }
            }
            None => return String::from("No children"),
        }
        child_names.push_str("]");
        return child_names;
    }

    // Consumes the String
    fn find(&self, n: String) -> Option<Rc<Node>> {
        match &self.children {
            Some(c) => {
                for child in c.borrow().iter() {
                    if child.name == n {
                        return Some(Rc::clone(child));
                    }
                }
                return None;
            }
            None => return None,
        }
    }

    fn parent_name(&self) -> String {
        match &self.parent {
            Some(p) => return String::clone(&p.name),
            None => return String::from("No parent"),
        }
    }

    #[allow(dead_code)]
    fn print_tree(&self) {
        println!("{self}");
        match &self.children {
            Some(c) => {
                for child in c.borrow().iter() {
                    child.print_tree();
                }
            }
            None => (),
        }
    }

    fn smallest_free(&self, n: i32) -> Option<i32> {
        match &self.children {
            Some(c) => {
                let mut smallest = self.total_size();
                for child in c.borrow().iter() {
                    match child.smallest_free(n) {
                        Some(i) => {
                            if i >= n && i < smallest {
                                smallest = i;
                            }
                        }
                        None => (),
                    }
                }
                return Some(smallest);
            }
            None => return None,
        }
    }

    fn sum_dirs_lte(&self, n: i32) -> i32 {
        match &self.children {
            Some(c) => {
                let mut sum = 0;
                if self.total_size_lte(n) {
                    sum += self.total_size();
                }
                for child in c.borrow().iter() {
                    sum += child.sum_dirs_lte(n);
                }
                return sum;
            }
            None => return 0,
        }
    }

    fn total_size(&self) -> i32 {
        let mut size = self.size;
        match &self.children {
            Some(c) => {
                for child in c.borrow().iter() {
                    size += child.total_size();
                }
            }
            None => return self.size,
        }
        return size;
    }

    fn total_size_lte(&self, n: i32) -> bool {
        return self.total_size() <= n;
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Name: {}, Size: {}, Parent: {}, Children: {})",
            self.name,
            self.total_size(),
            self.parent_name(),
            self.child_names()
        )
    }
}

pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();
    let head_node = Rc::new(Node {
        name: String::from("/"),
        size: 0,
        parent: None,
        children: Some(RefCell::new(Vec::new())),
    });
    let mut current = Rc::clone(&head_node);
    let mut first = true;
    for line in lines {
        if first {
            first = false;
            continue;
        }
        if line == "$ ls" {
            continue;
        }
        if line == "$ cd .." {
            // Change to current = current.parent
            current = Rc::clone(current.parent.as_ref().expect("Expected a parent"))
        } else if line.starts_with("$ cd ") {
            let (_, dirname) = line.split_once("$ cd ").unwrap();
            // Change to current = dirname
            current = current
                .find(dirname.to_string())
                .expect("Couldn't find dirname as a child of current");
        } else if line.starts_with("dir") {
            let (_, dirname) = line.split_once(" ").unwrap();
            // Create a new directory node and add it to current's children
            current.add_child(Rc::new(Node {
                name: String::from(dirname),
                size: 0,
                parent: Some(Rc::clone(&current)),
                children: Some(RefCell::new(Vec::new())),
            }));
        } else {
            let (size, dirname) = line.split_once(" ").unwrap();
            // Create a new file node and add it to current's children
            current.add_child(Rc::new(Node {
                name: String::from(dirname),
                size: size.parse::<i32>().expect("Couldn't decode &str into i32"),
                parent: Some(Rc::clone(&current)),
                children: None,
            }));
        }
    }

    let sum_dirs = head_node.sum_dirs_lte(100000);
    println!("Sum of dirs size <= 100,000: {sum_dirs}");
    const TOTAL: i32 = 70000000;
    let available: i32 = TOTAL - head_node.total_size();
    const NEEDED: i32 = 30000000;
    let smallest_free = head_node
        .smallest_free(NEEDED - available)
        .expect("Couldn't find smallest free");
    println!("Smallest directory deletion: {smallest_free}");
    return (sum_dirs, smallest_free);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_7.txt");
    assert_eq!(main(test_file), (95437, 24933642));
}
