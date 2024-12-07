pub fn part1(input: Vec<String>) -> String
{
    let mut total = 0;

    for line in input.iter() {
        let mut split: Vec<&str> = line.split_whitespace()
                                       .collect();

        let test_value: usize = split.first()
                                     .unwrap()
                                     [.. split.first().unwrap().len() -1]
                                     .parse::<usize>()
                                     .unwrap();

        split.remove(0);
        let the_rest: Vec<usize> = split.iter()
                                        .map(|n| n.parse::<usize>().unwrap())
                                        .collect();

        if does_crunch_one(test_value, &the_rest, *the_rest.get(0).unwrap(), 1) { total += test_value }
    }

    format!("{}", total)
}

pub fn part2(input: Vec<String>) -> String {
    let mut total = 0;

    for line in input.iter() {
        let mut split: Vec<&str> = line.split_whitespace()
                                       .collect();

        let test_value: usize = split.first()
                                     .unwrap()
                                     [.. split.first().unwrap().len() -1]
                                     .parse::<usize>()
                                     .unwrap();

        split.remove(0);
        let the_rest: Vec<usize> = split.iter()
                                        .map(|n| n.parse::<usize>().unwrap())
                                        .collect();

        if does_crunch_two(test_value, &the_rest, *the_rest.get(0).unwrap(), 1) { total += test_value }
    }

    format!("{}", total)
}

fn does_crunch_one(test_value: usize, the_rest: &Vec<usize>, current: usize, index: usize) -> bool
{
    if index == the_rest.len() {
        return current == test_value;
    }
    let next_val = the_rest.get(index).unwrap();

    does_crunch_one(test_value, the_rest, current + next_val, index + 1)
    || does_crunch_one(test_value, the_rest, current * next_val, index + 1)
}

fn does_crunch_two(test_value: usize, the_rest: &Vec<usize>, current: usize, index: usize) -> bool
{
    if index == the_rest.len() {
        return current == test_value;
    }
    let next_val = the_rest.get(index).unwrap();

    does_crunch_two(test_value, the_rest, current + next_val, index + 1)
    || does_crunch_two(test_value, the_rest, current * next_val, index + 1)
    || does_crunch_two(test_value, the_rest, concat(current, *next_val), index + 1)
}

fn concat(a: usize, b: usize) -> usize {
        let num_digits = b.to_string().len();
        a * 10_usize.pow(num_digits as u32) + b
}