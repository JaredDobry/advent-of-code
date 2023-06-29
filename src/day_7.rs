struct Node {
    name: String,
    size: i32,
    parent: Option<std::rc::Rc<Node>>,
    children: std::cell::RefCell<std::vec::Vec<Node>>,
}

impl Copy for Node {
    fn copy(&self) -> Node {
        return Node {
            name: self.name.clone(),
            size: self.size.clone(),
            parent: self.parent.clone(),
            children: self.children.borrow(),
        };
    }
}

impl Node {
    fn calculate_size(self) -> i32 {
        let mut size = self.size;
        for child in self.children.borrow().iter() {
            size += (*child).calculate_size();
        }
        return size;
    }

    fn find_child(self, name: &str) -> Option<std::rc::Rc<Node>> {
        for child in self.children.borrow().iter() {
            if (*child).name == name {
                return Some(std::rc::Rc::new(*child));
            }
        }
        return None;
    }
}

pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();
    let head_node = Node {
        name: "/".to_string(),
        size: 0,
        parent: None,
        children: std::cell::RefCell::new(std::vec::Vec::new()),
    };
    let mut current = std::rc::Rc::new(head_node);
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
            if current.parent.is_some() {
                current = current.as_ref().parent.clone().unwrap();
            }
            continue;
        }
        if line.starts_with("$ cd ") {
            let (_, dirname) = line.split_once("$ cd ").unwrap();
            let result = current.as_ref().find_child(dirname);
        }
        if line.starts_with("dir") {
            let (_, dirname) = line.split_once(" ").unwrap();
            current.children.borrow_mut().push(Node {
                name: dirname.to_string(),
                size: 0,
                parent: Some(current.clone()),
                children: std::cell::RefCell::new(std::vec::Vec::new()),
            });
            continue;
        }
    }

    return (0, 0);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_7.txt");
    assert_eq!(main(test_file), (0, 0));
}
