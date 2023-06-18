fn crate_mover_9000(lines: std::str::Lines<'_>) -> String {
    let mut stacks = std::vec::Vec::new();
    let mut first = true;
    for line in lines {
        let n = (line.len() + 1) / 4;
        if first {
            for _ in 0..n {
                stacks.push(std::collections::VecDeque::<char>::new());
            }
            first = false;
        }
        if line.find('[').is_some() {
            for i in 0..n {
                if line.chars().nth(i * 4).unwrap() == '[' {
                    stacks[i].push_front(line.chars().nth(i * 4 + 1).unwrap());
                }
            }
        } else if line.find("move").is_some() {
            let mut instructions: [usize; 3] = [0; 3]; //move n from idx to idx
            let mut i = 0;
            for s in line.split(" ") {
                if s.chars().all(|c| c.is_numeric()) {
                    instructions[i] = s.parse::<usize>().unwrap();
                    i += 1;
                }
            }
            for _ in 0..instructions[0] {
                let c = stacks[instructions[1] - 1].pop_back().unwrap();
                stacks[instructions[2] - 1].push_back(c);
            }
        }
    }

    let mut top_crates = String::new();
    for stack in stacks {
        if stack.back().is_some() {
            top_crates.push(*stack.back().unwrap());
        } else {
            top_crates += " ";
        }
    }
    return top_crates;
}

fn crate_mover_9001(lines: std::str::Lines<'_>) -> String {
    let mut stacks = std::vec::Vec::new();
    let mut first = true;
    for line in lines {
        let n = (line.len() + 1) / 4;
        if first {
            for _ in 0..n {
                stacks.push(std::collections::VecDeque::<char>::new());
            }
            first = false;
        }
        if line.find('[').is_some() {
            for i in 0..n {
                if line.chars().nth(i * 4).unwrap() == '[' {
                    stacks[i].push_front(line.chars().nth(i * 4 + 1).unwrap());
                }
            }
        } else if line.find("move").is_some() {
            let mut instructions: [usize; 3] = [0; 3]; //move n from idx to idx
            let mut i = 0;
            for s in line.split(" ") {
                if s.chars().all(|c| c.is_numeric()) {
                    instructions[i] = s.parse::<usize>().unwrap();
                    i += 1;
                }
            }

            if instructions[0] == 1 {
                let c = stacks[instructions[1] - 1].pop_back().unwrap();
                stacks[instructions[2] - 1].push_back(c);
            } else {
                let mut move_group = std::vec::Vec::new();
                for _ in 0..instructions[0] {
                    move_group.push(stacks[instructions[1] - 1].pop_back().unwrap());
                }
                move_group.reverse();
                stacks[instructions[2] - 1].extend(move_group);
            }
        }
    }

    let mut top_crates = String::new();
    for stack in stacks {
        if stack.back().is_some() {
            top_crates.push(*stack.back().unwrap());
        } else {
            top_crates += " ";
        }
    }
    return top_crates;
}

pub fn main(file: std::path::PathBuf) -> (String, String) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let result_9000 = crate_mover_9000(content.lines());
    let result_9001 = crate_mover_9001(content.lines());
    println!("CrateMover 9000: {result_9000}");
    println!("CrateMover 9001: {result_9001}");
    return (result_9000, result_9001);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_5.txt");
    assert_eq!(main(test_file), ("CMZ".to_string(), "MCD".to_string()));
}
