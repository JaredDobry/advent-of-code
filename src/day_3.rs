const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(c: char) -> i32 {
    if c.is_lowercase() {
        return LOWERCASE.find(c).unwrap() as i32 + 1;
    } else {
        return UPPERCASE.find(c).unwrap() as i32 + 27;
    }
}

fn find_common(l: &str, r: &str) -> char {
    let mut l_set = std::collections::HashSet::new();
    let mut r_set = std::collections::HashSet::new();
    for c in l.chars() {
        l_set.insert(c);
    }
    for c in r.chars() {
        r_set.insert(c);
    }
    let mut common = l_set.intersection(&r_set);
    return *common.next().unwrap();
}

fn find_common_multi(strings: &Vec<&str>) -> char {
    let mut sets = std::vec::Vec::new();
    for s in strings {
        let mut set = std::collections::HashSet::new();
        for c in s.chars() {
            set.insert(c);
        }
        sets.push(set);
    }

    let mut common = sets[0].to_owned();
    let mut first = true;
    for set in sets {
        if first {
            first = false;
        } else {
            let intersect = common.intersection(&set);
            let mut s = std::collections::HashSet::new();
            for c in intersect {
                s.insert(*c);
            }
            common = s;
        }
    }

    return common.into_iter().next().unwrap();
}

pub fn main(file: std::path::PathBuf) -> (i32, i32) {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines = content.lines();
    let mut sum = 0;
    let mut group_sum = 0;
    let mut group: Vec<&str> = std::vec::Vec::new();
    for line in lines {
        let (l, r) = line.split_at(line.len() / 2);
        let common = find_common(l, r);
        sum += priority(common);

        group.push(line);
        if group.len() == 3 {
            group_sum += priority(find_common_multi(&group));
            group.clear();
        }
    }

    println!("Sum of priorities: {sum}");
    println!("Sum of group badge priorites: {group_sum}");
    return (sum, group_sum);
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_3.txt");
    assert_eq!(main(test_file), (157, 70));
}
