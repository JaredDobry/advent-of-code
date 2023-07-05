fn check_cycle(cycle: i32, register: i32) -> i32 {
    if cycle == 20 || (cycle - 20) % 40 == 0 {
        return register * cycle;
    }
    return 0;
}

fn draw_pixel(cycle: i32, register: i32) {
    if cycle % 40 == 0 {
        print!("\n");
    }
    let line_idx = cycle % 40;
    if line_idx >= register - 1 && line_idx <= register + 1 {
        print!("#");
    } else {
        print!(".");
    }
}

pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();
    let mut cycle = 0;
    let mut register = 1;
    let mut sum = 0;
    for line in lines {
        if line == "noop" {
            draw_pixel(cycle, register);
            cycle += 1;
            sum += check_cycle(cycle, register);
        } else if line.starts_with("addx") {
            let (_, v_str) = line.split_once(" ").expect("Couldn't split addx");
            let v = v_str.parse::<i32>().expect("Couldn't parse &str to i32");
            for _ in 0..2 {
                draw_pixel(cycle, register);
                cycle += 1;
                sum += check_cycle(cycle, register);
            }
            register += v;
        }
    }

    println!("\nnt Total signal strength: {sum}");

    return (sum, 0);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_10.txt");
    assert_eq!(main(test_file), (13140, 0));
}
