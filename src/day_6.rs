fn unique(s: &str) -> bool {
    for i in 0..s.len() - 1 {
        if s[i + 1..].find(s.chars().nth(i).unwrap()).is_some() {
            return false;
        }
    }
    return true;
}

fn marker(s: &str, n: usize) -> usize {
    assert!(n <= s.len());
    let mut l = 0;
    let mut r = n - 1;
    while r < s.len() {
        if unique(&s[l..=r]) {
            return r + 1;
        }
        l += 1;
        r += 1;
    }
    return 0;
}

pub fn main(file: std::path::PathBuf) -> (usize, usize) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();
    let mut packet = 0;
    let mut message = 0;
    for line in lines {
        packet = marker(line, 4);
        message = marker(line, 14);
        println!("Packet marker: {packet}");
        println!("Message marker: {message}")
    }

    return (packet, message);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_6.txt");
    assert_eq!(main(test_file), (11, 26));
}
