mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

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
    } else if args.day == 8 {
        day_8::main(args.file);
    } else if args.day == 9 {
        day_9::main(args.file);
    } else if args.day == 10 {
        day_10::main(args.file);
    } else if args.day == 11 {
        day_11::main(args.file);
    } else if args.day == 12 {
        day_12::main(args.file);
    }
}
