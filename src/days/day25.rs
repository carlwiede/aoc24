pub fn part1(input: Vec<String>) -> String
{
    let (locks, keys) = parse_input(&input);
    let mut matching = 0;

    for key in &keys {
        for lock in &locks {
            if key_matches_lock(key, lock) { matching += 1 }
        }
    }

    format!("{}", matching)
}

pub fn part2(_input: Vec<String>) -> String {
    // Your Part 2 solution logic
    "Part 2 not implemented yet".to_string()
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>)
{
    let mut locks: Vec<Vec<usize>> = vec![];
    let mut keys: Vec<Vec<usize>> = vec![];

    let mut top_row = 0;

    for (i, line) in input.iter().enumerate() {
        if line.is_empty() {
            if input[top_row] == "#####" {
                locks.push(get_key_lock_value(&input[top_row..i]));
            } else {
                keys.push(get_key_lock_value(&input[top_row..i]));
            }
            top_row = i+1;
        }
    }

    (locks, keys)
}

fn get_key_lock_value(grid: &[String]) -> Vec<usize>
{
    let mut values = vec![];

    for x in 0..grid[0].len() {
        let mut column = 0;
        for y in 0..grid.len() {
            if grid[y].chars().nth(x).unwrap() == '#' { column += 1 }
        }
        values.push(column - 1);
    }

    values
}

fn key_matches_lock(key: &Vec<usize>, lock: &Vec<usize>) -> bool
{
    let mut index = 0;

    while index < 5 {
        if key[index] + lock[index] >= 6 { return false }
        index += 1;
    }

    true
}