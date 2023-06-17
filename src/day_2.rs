const THREW_ROCK: i32 = 1;
const THREW_PAPER: i32 = 2;
const THREW_SCISSORS: i32 = 3;
const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

fn score(opponent: char, you: char) -> i32 {
    // Rock vs. Rock
    if opponent == 'A' && you == 'X' {
        return THREW_ROCK + DRAW;
    }
    // Rock vs. Paper
    else if opponent == 'A' && you == 'Y' {
        return THREW_PAPER + WIN;
    }
    // Rock vs. Scissors
    else if opponent == 'A' && you == 'Z' {
        return THREW_SCISSORS + LOSS;
    }
    // Paper vs. Rock
    else if opponent == 'B' && you == 'X' {
        return THREW_ROCK + LOSS;
    }
    // Paper vs. Paper
    else if opponent == 'B' && you == 'Y' {
        return THREW_PAPER + DRAW;
    }
    // Paper vs. Scissors
    else if opponent == 'B' && you == 'Z' {
        return THREW_SCISSORS + WIN;
    }
    // Scissors vs. Rock
    else if opponent == 'C' && you == 'X' {
        return THREW_ROCK + WIN;
    }
    // Scissors vs. Paper
    else if opponent == 'C' && you == 'Y' {
        return THREW_PAPER + LOSS;
    }
    // Scissors vs. Scissors
    else if opponent == 'C' && you == 'Z' {
        return THREW_SCISSORS + DRAW;
    } else {
        println!("Unsupported game state: {} {}", opponent, you);
        return 0;
    }
}

fn determine_throw(opponent: char, you: char) -> char {
    // Rock Loss
    if opponent == 'A' && you == 'X' {
        return 'Z';
    }
    // Rock Draw
    else if opponent == 'A' && you == 'Y' {
        return 'X';
    }
    // Rock Win
    else if opponent == 'A' && you == 'Z' {
        return 'Y';
    }
    // Paper Loss
    else if opponent == 'B' && you == 'X' {
        return 'X';
    }
    // Paper Draw
    else if opponent == 'B' && you == 'Y' {
        return 'Y';
    }
    // Paper Win
    else if opponent == 'B' && you == 'Z' {
        return 'Z';
    }
    // Scissors Loss
    else if opponent == 'C' && you == 'X' {
        return 'Y';
    }
    // Scissors Draw
    else if opponent == 'C' && you == 'Y' {
        return 'Z';
    }
    // Scissors Win
    else if opponent == 'C' && you == 'Z' {
        return 'X';
    } else {
        println!("Unsupported game state: {} {}", opponent, you);
        return 'E';
    }
}

pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();
    let mut total_score = 0;
    let mut strategic_score = 0;
    for line in lines {
        total_score += score(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
        strategic_score += score(
            line.chars().nth(0).unwrap(),
            determine_throw(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()),
        )
    }
    println!("Total: {}", total_score);
    println!("Strategic: {}", strategic_score);
    return (total_score, strategic_score);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_2.txt");
    assert_eq!(main(test_file), (15, 12));
}
