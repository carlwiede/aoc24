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
        5 => {
            println!("Day 5, Part 1: {}", days::day5::part1(input.clone()));
            println!("Day 5, Part 2: {}", days::day5::part2(input));
        }
        6 => {
            println!("Day 6, Part 1: {}", days::day6::part1(input.clone()));
            println!("Day 6, Part 2: {}", days::day6::part2(input));
        }
        7 => {
            println!("Day 7, Part 1: {}", days::day7::part1(input.clone()));
            println!("Day 7, Part 2: {}", days::day7::part2(input));

        }
        8 => {
            println!("Day 8, Part 1: {}", days::day8::part1(input.clone()));
            println!("Day 8, Part 2: {}", days::day8::part2(input));

        }
        9 => {
            println!("Day 9, Part 1: {}", days::day9::part1(input.clone()));
            println!("Day 9, Part 2: {}", days::day9::part2(input));

        }
        10 => {
            println!("Day 10, Part 1: {}", days::day10::part1(input.clone()));
            println!("Day 10, Part 2: {}", days::day10::part2(input));

        }
        11 => {
            println!("Day 11, Part 1: {}", days::day11::part1(input.clone()));
            println!("Day 11, Part 2: {}", days::day11::part2(input));

        }
        12 => {
            println!("Day 12, Part 1: {}", days::day12::part1(input.clone()));
            println!("Day 12, Part 2: {}", days::day12::part2(input));
        }
        13 => {
            println!("Day 13, Part 1: {}", days::day13::part1(input.clone()));
            println!("Day 13, Part 2: {}", days::day13::part2(input));
        }
        14 => {
            println!("Day 14, Part 1: {}", days::day14::part1(input.clone()));
            println!("Day 14, Part 2: {}", days::day14::part2(input));
        }
        15 => {
            println!("Day 15, Part 1: {}", days::day15::part1(input.clone()));
            println!("Day 15, Part 2: {}", days::day15::part2(input));
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
