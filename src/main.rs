mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

struct Cli {
    day: i8,
    file: std::path::PathBuf,
}

fn main() {
    let day = std::env::args().nth(1).expect("No day specified.");
    let file = std::env::args().nth(2).expect("No input file specified.");
    let args = Cli {
        day: day.parse::<i8>().unwrap(),
        file: std::path::PathBuf::from(file),
    };
    if args.day == 1 {
        day_1::main(args.file);
    } else if args.day == 2 {
        day_2::main(args.file);
    } else if args.day == 3 {
        day_3::main(args.file);
    } else if args.day == 4 {
        day_4::main(args.file);
    } else if args.day == 5 {
        day_5::main(args.file);
    } else if args.day == 6 {
        day_6::main(args.file);
    } else if args.day == 7 {
        day_7::main(args.file);
    }
}
