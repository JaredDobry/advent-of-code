use std::collections::HashSet;

fn is_touching(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    return tail.0 >= head.0 - 1
        && tail.0 <= head.0 + 1
        && tail.1 >= head.1 - 1
        && tail.1 <= head.1 + 1;
}

fn find_touching(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    //Fight stick format
    //1
    if head.0 < tail.0 && head.1 < tail.1 {
        return (tail.0 - 1, tail.1 - 1);
    }
    //2
    else if head.0 == tail.0 && head.1 < tail.1 {
        return (tail.0, tail.1 - 1);
    }
    //3
    else if head.0 > tail.0 && head.1 < tail.1 {
        return (tail.0 + 1, tail.1 - 1);
    }
    //4
    else if head.0 < tail.0 && head.1 == tail.1 {
        return (tail.0 - 1, tail.1);
    }
    //6
    else if head.0 > tail.0 && head.1 == tail.1 {
        return (tail.0 + 1, tail.1);
    }
    //7
    else if head.0 < tail.0 && head.1 > tail.1 {
        return (tail.0 - 1, tail.1 + 1);
    }
    //8
    else if head.0 == tail.0 && head.1 > tail.1 {
        return (tail.0, tail.1 + 1);
    }
    //9
    else if head.0 > tail.0 && head.1 > tail.1 {
        return (tail.0 + 1, tail.1 + 1);
    }
    //5
    else {
        return tail.clone();
    }
}

fn print_chain(chain: &Vec<(i32, i32)>) {
    let mut x_bound = (0, 0);
    let mut y_bound = (0, 0);
    for knot in chain {
        if knot.0 < x_bound.0 {
            x_bound.0 = knot.0;
        }
        if knot.0 > x_bound.1 {
            x_bound.1 = knot.0;
        }
        if knot.1 < y_bound.0 {
            y_bound.0 = knot.1;
        }
        if knot.1 > y_bound.1 {
            y_bound.1 = knot.1;
        }
    }

    for y in y_bound.0..=y_bound.1 {
        let mut line = String::from("");
        for x in x_bound.0..=x_bound.1 {
            for i in 0..chain.len() {
                if chain[i] == (x, y) {
                    line.push_str(i.to_string().as_str());
                } else {
                    line.push('.');
                }
            }
        }
        println!("{}", line);
    }
}

pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();

    let mut visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert((0, 0));

    let mut visited_p2 = HashSet::new();
    let mut chain = Vec::new();
    for _ in 0..10 {
        chain.push((0, 0));
    }
    visited_p2.insert((0, 0));

    print_chain(&chain);

    for line in lines {
        let (dir, steps) = line.split_once(" ").expect("Couldn't split input");
        let step_count = steps
            .parse::<u32>()
            .expect("Couldn't parse digit string to i32");
        for _ in 0..step_count {
            if dir == "L" {
                head.0 -= 1;
                chain[0].0 -= 1;
            } else if dir == "R" {
                head.0 += 1;
                chain[0].0 += 1;
            } else if dir == "U" {
                head.1 += 1;
                chain[0].1 += 1;
            } else if dir == "D" {
                head.1 -= 1;
                chain[0].1 -= 1;
            }
            if !is_touching(&head, &tail) {
                tail = find_touching(&head, &tail);
                visited.insert(tail);
            }
            for i in 1..chain.len() {
                if !is_touching(&chain[i - 1], &chain[i]) {
                    chain[i] = find_touching(&chain[i - 1], &chain[i]);
                    if i == chain.len() - 1 {
                        visited.insert(chain[i]);
                    }
                }
            }
        }
        print_chain(&chain);
    }

    let num_visited = visited.len().try_into().unwrap();
    println!(
        "Number of unique positions visited by tail (p1): {}",
        num_visited
    );

    let num_visited_p2 = visited_p2.len().try_into().unwrap();
    println!(
        "Number of unique positions visited by tail (p2): {}",
        num_visited_p2
    );

    return (num_visited, num_visited_p2);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_9.txt");
    let answer = main(test_file);
    assert_eq!(answer.0, 13);
}

#[test]
fn check_answer_valid_p2() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_9_p2.txt");
    let answer = main(test_file);
    assert_eq!(answer.1, 36);
}
