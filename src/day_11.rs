#[derive(Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Square,
}

#[derive(Clone, Copy)]
struct Test {
    divisor: u64,
    true_id: usize,
    false_id: usize,
}

impl Test {
    fn do_test(&self, worry: &u64) -> usize {
        if worry % self.divisor == 0 {
            return self.true_id;
        }
        return self.false_id;
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    operand: Option<u64>,
    test: Test,
    inspections: u64,
}

impl Monkey {
    fn clear(&mut self) {
        self.inspections += u64::try_from(self.items.len()).expect("usize couldn't fit into u32");
        self.items.clear();
    }

    fn do_operation(&self, old: &u64) -> u64 {
        match self.operation {
            Operation::Add => return old + self.operand.expect("Called Add with None operand"),
            Operation::Multiply => {
                return old * self.operand.expect("Called Multiply with None operand")
            }
            Operation::Square => return old * old,
        }
    }

    fn push(&mut self, worry: &u64) {
        self.items.push(worry.clone());
    }
}

pub fn main(file: std::path::PathBuf) -> u64 {
    let content = std::fs::read_to_string(&file).expect("Couldn't read input file.");
    let lines: Vec<&str> = content.lines().collect();
    let mut divisor_mul = 1;
    let mut monkeys: Vec<Monkey> = Vec::new();
    let chunks = lines.chunks(7);
    for chunk in chunks {
        let operation = match chunk[2].contains('+') {
            true => Operation::Add,
            false => match chunk[2].matches("old").count() > 1 {
                true => Operation::Square,
                false => Operation::Multiply,
            },
        };
        let operand = match operation {
            Operation::Add | Operation::Multiply => Some(
                chunk[2]
                    .split(' ')
                    .last()
                    .expect("Couldn't split operation")
                    .parse::<u64>()
                    .expect("Couldn't parse u64"),
            ),
            Operation::Square => None,
        };
        let divisor = chunk[3]
            .split(' ')
            .last()
            .expect("Couldn't split test")
            .parse::<u64>()
            .expect("Couldn't parse u64");
        if divisor_mul % divisor != 0 {
            divisor_mul *= divisor;
        }
        monkeys.push(Monkey {
            items: chunk[1]
                .split_once(": ")
                .expect("Couldn't split starting items")
                .1
                .split(", ")
                .map(|x| x.parse::<u64>().expect("Couldn't parse u64"))
                .collect(),
            operation: operation,
            operand: operand,
            test: Test {
                divisor: divisor,
                true_id: chunk[4]
                    .split(' ')
                    .last()
                    .expect("Couldn't split true throw")
                    .parse::<usize>()
                    .expect("Couldn't parse usize"),
                false_id: chunk[5]
                    .split(' ')
                    .last()
                    .expect("Couldn't split false throw")
                    .parse::<usize>()
                    .expect("Couldn't parse usize"),
            },
            inspections: 0,
        })
    }

    println!("Starting with divisor_mul: {divisor_mul}");

    for round in 0..10000 {
        if round % 1000 == 0 {
            println!("Round {round}");
        }
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for mut item in monkey.items.iter().copied() {
                item %= divisor_mul;
                let new_worry = monkey.do_operation(&item);
                let id = monkey.test.do_test(&new_worry);
                monkeys[id].push(&new_worry);
            }
            monkeys[i].clear();
        }
    }

    monkeys.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    let most_active = monkeys
        .chunks(2)
        .last()
        .expect("Couldn't get last 2 monkeys");

    let monkey_business: u64 = most_active[0].inspections * most_active[1].inspections;

    println!("Monkey business level: {}", monkey_business);

    return monkey_business;
}

#[test]
fn check_answer_valid() {
    let mut test_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_file.push("resources/test/day_11.txt");
    let answer: u64 = 2713310158;
    assert_eq!(main(test_file), answer);
}
