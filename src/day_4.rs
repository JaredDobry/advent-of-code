fn contains(l_lower: i32, l_upper: i32, r_lower: i32, r_upper: i32) -> bool {
    // l contain r
    if l_lower <= r_lower && l_upper >= r_upper {
        return true;
    }
    // r contain l
    else if r_lower <= l_lower && r_upper >= l_upper {
        return true;
    }
    return false;
}

fn overlaps(l_lower: i32, l_upper: i32, r_lower: i32, r_upper: i32) -> bool {
    // L left overlaps R
    if l_lower <= r_lower && l_upper >= r_lower {
        return true;
    }
    // L right overlaps R
    else if l_lower <= r_upper && l_upper >= r_upper {
        return true;
    }
    // R left overlaps L
    else if r_lower <= l_lower && r_upper >= l_lower {
        return true;
    }
    // R right overlaps L
    else if r_lower <= l_upper && r_upper >= l_upper {
        return true;
    }
    return false;
}

pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();
    let mut contain_count = 0;
    let mut overlap_count = 0;
    for line in lines {
        let (l, r) = line.split_once(',').unwrap();
        let (l_lower, l_upper) = l.split_once('-').unwrap();
        let (r_lower, r_upper) = r.split_once('-').unwrap();
        if contains(
            l_lower.parse::<i32>().unwrap(),
            l_upper.parse::<i32>().unwrap(),
            r_lower.parse::<i32>().unwrap(),
            r_upper.parse::<i32>().unwrap(),
        ) {
            contain_count += 1;
        }
        if overlaps(
            l_lower.parse::<i32>().unwrap(),
            l_upper.parse::<i32>().unwrap(),
            r_lower.parse::<i32>().unwrap(),
            r_upper.parse::<i32>().unwrap(),
        ) {
            overlap_count += 1;
        }
    }

    println!("Contain count: {contain_count}");
    println!("Overlap count: {overlap_count}");

    return (contain_count, overlap_count);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_4.txt");
    assert_eq!(main(test_file), (2, 4));
}
