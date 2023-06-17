pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();
    let mut calories = Vec::new();
    let mut current_elf = 0;
    for line in lines {
        if line.len() == 0 { current_elf += 1; }
        else { 
            if calories.len() <= current_elf { calories.push(0); }
            calories[current_elf] += line.parse::<i32>().unwrap();
         }
    }
    calories.sort();
    calories.reverse();
    let max = calories[0];
    let top_3 = max + calories[1] + calories[2];
    println!("Max: {}", max);
    println!("Sum of top 3: {}", top_3);
    return (max, top_3);
}

#[test]
fn check_answer_valid() {
    println!("{}", env!("CARGO_MANIFEST_DIR"));
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_1.txt");
    assert_eq!(main(test_file), (24000, 45000));
}