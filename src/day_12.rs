use std::collections::HashMap;

const CHAR_OFFSET: u32 = 'a' as u32;

#[derive(Clone, Copy, Debug)]
struct DijkstraNode {
    height: u32,
    distance: u32,
    visited: bool,
}

fn can_visit(n: &DijkstraNode, other: &DijkstraNode) -> bool {
    return n.height + 1 >= other.height;
}

fn next_dijkstra(m: &HashMap<(usize, usize), DijkstraNode>) -> (usize, usize) {
    let next = m
        .into_iter()
        .filter(|x| !x.1.visited)
        .min_by(|a, b| a.1.distance.cmp(&b.1.distance));
    match next {
        Some(n) => return *n.0,
        None => panic!("Couldn't find a node to visit"),
    }
}

fn dijkstra(
    m: &HashMap<(usize, usize), DijkstraNode>,
    start: (usize, usize),
    goal: (usize, usize),
) -> u32 {
    let mut map = m.clone();
    let mut i = start;
    map.get_mut(&i)
        .expect("Couldn't find start node in map")
        .distance = 0;
    loop {
        let current = map.get(&i).expect("Couldn't find node in map").clone();
        if current.distance == u32::MAX {
            // No valid way to goal
            return u32::MAX;
        }
        if i == goal {
            return current.distance;
        }

        // Top
        if i.0 > 0 {
            match map.get_mut(&(i.0 - 1, i.1)) {
                Some(n) => {
                    n.distance = n.distance.min(match can_visit(&current, n) {
                        true => current.distance + 1,
                        false => u32::MAX,
                    })
                }
                None => (),
            };
        }

        // Bottom
        match map.get_mut(&(i.0 + 1, i.1)) {
            Some(n) => {
                n.distance = n.distance.min(match can_visit(&current, n) {
                    true => current.distance + 1,
                    false => u32::MAX,
                })
            }
            None => (),
        };

        // Left
        if i.1 > 0 {
            match map.get_mut(&(i.0, i.1 - 1)) {
                Some(n) => {
                    n.distance = n.distance.min(match can_visit(&current, n) {
                        true => current.distance + 1,
                        false => u32::MAX,
                    })
                }
                None => (),
            };
        }

        // Right
        match map.get_mut(&(i.0, i.1 + 1)) {
            Some(n) => {
                n.distance = n.distance.min(match can_visit(&current, n) {
                    true => current.distance + 1,
                    false => u32::MAX,
                })
            }
            None => (),
        };

        map.get_mut(&i).expect("Couldn't find node in map").visited = true;
        i = next_dijkstra(&map);
    }
}

pub fn main(file: std::path::PathBuf) -> (u32, u32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines: Vec<&str> = content.lines().collect();

    let mut map = HashMap::new();
    let mut start = None;
    let mut goal = None;

    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            let c = lines[row].chars().nth(col).expect("Invalid column index");
            if c == 'S' {
                start = Some((row, col));
                map.insert(
                    (row, col),
                    DijkstraNode {
                        height: 0,
                        distance: u32::MAX,
                        visited: false,
                    },
                );
            } else if c == 'E' {
                goal = Some((row, col));
                map.insert(
                    (row, col),
                    DijkstraNode {
                        height: 'z' as u32 - CHAR_OFFSET,
                        distance: u32::MAX,
                        visited: false,
                    },
                );
            } else {
                map.insert(
                    (row, col),
                    DijkstraNode {
                        height: c as u32 - CHAR_OFFSET,
                        distance: u32::MAX,
                        visited: false,
                    },
                );
            }
        }
    }

    let part1 = dijkstra(
        &map,
        start.expect("Couldn't find start node"),
        goal.expect("Couldn't find goal node"),
    );
    println!("Part 1: {}", part1);

    let mut part2 = part1;
    for n in map.iter().filter(|x| x.1.height == 0).into_iter() {
        part2 = part2.min(dijkstra(&map, *n.0, goal.expect("Couldn't find goal node")));
    }
    println!("Part 2: {}", part2);

    return (part1, part2);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_12.txt");
    assert_eq!(main(test_file), (31, 29));
}
