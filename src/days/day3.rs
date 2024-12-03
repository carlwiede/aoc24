use regex::Regex;

pub fn part1(input: Vec<String>) -> String {

    let mut total = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for captures in re.captures_iter(input.first().unwrap()) {
        let factor1: i32 = captures[1].parse().unwrap();
        let factor2: i32 = captures[2].parse().unwrap();
        total += factor1 * factor2;
    }

    format!("{}", total)
}

pub fn part2(input: Vec<String>) -> String {

    let mut total = 0;
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_dont_re = Regex::new(r"don't\(\).*?(do\(\)|$)").unwrap();

    let mod_input = do_dont_re.replace_all(input.first().unwrap(), "").to_string();
    for captures in mul_re.captures_iter(&mod_input) {
        let factor1: i32 = captures[1].parse().unwrap();
        let factor2: i32 = captures[2].parse().unwrap();
        total += factor1 * factor2;
    }

    format!("{}", total)
}
