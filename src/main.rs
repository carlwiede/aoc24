use std::env;
mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <day>");
        std::process::exit(1);
    }

    let day = args[1].parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Invalid day number: {}", args[1]);
        std::process::exit(1);
    });

    let input = load_input(day);

    match day {
        1 => {
            println!("Day 1, Part 1: {}", days::day1::part1(input.clone()));
            println!("Day 1, Part 2: {}", days::day1::part2(input));
        }
        2 => {
            println!("Day 2, Part 1: {}", days::day2::part1(input.clone()));
            println!("Day 2, Part 2: {}", days::day2::part2(input));
        }
        3 => {
            println!("Day 3, Part 1: {}", days::day3::part1(input.clone()));
            println!("Day 3, Part 2: {}", days::day3::part2(input));
        }
        4 => {
            println!("Day 4, Part 1: {}", days::day4::part1(input.clone()));
            println!("Day 4, Part 2: {}", days::day4::part2(input));
        }
        _ => {
            eprintln!("Day {} is not implemented yet.", day);
        }
    }
}

fn load_input(day: usize) -> Vec<String> {
    let filename = format!("src/inputs/day{}.txt", day);
    std::fs::read_to_string(filename.clone())
        .unwrap_or_else(|_| {
            eprintln!("Failed to load input file: {}", filename);
            std::process::exit(1);
        })
        .lines()
        .map(String::from)
        .collect()
}
