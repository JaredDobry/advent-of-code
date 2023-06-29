use std::cell::RefCell;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Tree {
    height: u32,
    visible: RefCell<bool>,
}

impl Tree {
    fn see(&self) {
        *self.visible.borrow_mut() = true;
    }
}

fn num_visible(rows: &Vec<Vec<Tree>>) -> i32 {
    let mut n = 0;
    for r in rows {
        for t in r {
            if *t.visible.borrow() {
                n += 1;
            }
        }
    }
    return n;
}

fn left_visible(rows: &Vec<Vec<Tree>>) {
    for row in rows {
        for i in 0..row.len() {
            match row[0..i].iter().max() {
                Some(max) => {
                    if row[i].height > max.height {
                        row[i].see();
                    }
                }
                None => {
                    row[i].see();
                }
            }
        }
    }
}

fn right_visible(rows: &Vec<Vec<Tree>>) {
    for row in rows {
        for i in (0..row.len()).rev() {
            match row[i + 1..row.len()].iter().max() {
                Some(max) => {
                    if row[i].height > max.height {
                        row[i].see();
                        if row[i].height == 9 {
                            break;
                        }
                    }
                }
                None => {
                    row[i].see();
                    if row[i].height == 9 {
                        break;
                    }
                }
            }
        }
    }
}

fn top_visible(rows: &Vec<Vec<Tree>>, x: usize, y: usize) {
    for col in 0..x {
        let mut col_vals = Vec::new();
        for row in 0..y {
            match col_vals.iter().max() {
                Some(max) => {
                    if rows[row][col].height > *max {
                        rows[row][col].see();
                        if rows[row][col].height == 9 {
                            break;
                        }
                    }
                }
                None => {
                    rows[row][col].see();
                    if rows[row][col].height == 9 {
                        break;
                    }
                }
            }
            col_vals.push(rows[row][col].height);
        }
    }
}

fn bottom_visible(rows: &Vec<Vec<Tree>>, x: usize, y: usize) {
    for col in 0..x {
        let mut col_vals = Vec::new();
        for row in (0..y).rev() {
            match col_vals.iter().max() {
                Some(max) => {
                    if rows[row][col].height > *max {
                        rows[row][col].see();
                        if rows[row][col].height == 9 {
                            break;
                        }
                    }
                }
                None => {
                    rows[row][col].see();
                    if rows[row][col].height == 9 {
                        break;
                    }
                }
            }
            col_vals.push(rows[row][col].height);
        }
    }
}

fn print_visible(rows: &Vec<Vec<Tree>>) {
    for row in rows {
        let mut s = String::new();
        for t in row {
            if *t.visible.borrow() {
                s.push_str("1");
            } else {
                s.push_str("0");
            }
        }
        println!("{}", s);
    }
}

pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();

    let mut rows = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Tree {
                height: c.to_digit(10).expect("Non digit in input"),
                visible: RefCell::new(false),
            });
        }
        rows.push(row);
    }

    let x = rows[0].len();
    let y = rows.len();

    //outside_visible(&rows, x, y);
    left_visible(&rows);
    right_visible(&rows);
    top_visible(&rows, x, y);
    bottom_visible(&rows, x, y);

    print_visible(&rows);

    let visible = num_visible(&rows);
    println!("Number of outside trees: {}", 2 * x + 2 * (y - 2));
    println!("Number of trees visible: {visible}");

    return (visible, 0);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_8.txt");
    assert_eq!(main(test_file), (21, 0));
}
