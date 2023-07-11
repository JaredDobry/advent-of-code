pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines: Vec<&str> = content.lines().collect();
    let pairs: Vec<(&str, &str)> = lines
        .splitn(2, |s| !s.is_empty())
        .map(|p| (p[0], p[1]))
        .collect();

    dbg!(pairs);

    return (0, 0);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_13.txt");
    assert_eq!(main(test_file), (0, 0));
}
