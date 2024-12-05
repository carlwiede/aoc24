use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> String {

    let rules = parse_rules(&input);
    let mut valid_updates: Vec<Vec<i32>> = Vec::new();

    for line in input.iter().rev() {
        if line.is_empty() { break }

        let nums: Vec<i32> = parse_update(line);
        if is_valid(&nums, &rules) {
            valid_updates.push(nums);
        }
    }

    format!("{}", sum_middlemost(&valid_updates))
}

pub fn part2(input: Vec<String>) -> String {

    let rules = parse_rules(&input);
    let mut disordered_updates: Vec<Vec<i32>> = Vec::new();
    let mut fixed_updates: Vec<Vec<i32>> = Vec::new();

    for line in input.iter().rev() {
        if line.is_empty() { break }

        let nums: Vec<i32> = parse_update(line);
        if !is_valid(&nums, &rules) {
            disordered_updates.push(nums);
        }
    }

    for update in disordered_updates.iter() {
        let mut please_fix_me = update.clone();

        while !is_valid(&please_fix_me, &rules) {
            please_fix_me = attempt_to_fix(&please_fix_me, &rules).unwrap();
        }

        fixed_updates.push(please_fix_me);
    }

    format!("{}", sum_middlemost(&fixed_updates))
}

fn parse_rules(input: &Vec<String>) -> HashMap<i32, Vec<i32>> {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in input.iter() {
        if line.is_empty() { break }
        let (l, r): (i32, i32) = {
            let parts: Vec<&str> = line.split('|').collect();
            (
                parts[0].parse().unwrap(),
                parts[1].parse().unwrap(),
            )
        };
        rules.entry(l).or_insert_with(Vec::new).push(r);
    }
    rules
}

fn parse_update(in_str: &String) -> Vec<i32>{
    return in_str.split(",")
                 .map(|n| n.parse::<i32>().unwrap())
                 .collect();
}

fn is_valid(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, &n) in update.iter().enumerate() {
        for &prev in &update[..i] {
            match rules.get(&n) {
                Some(vec) => {
                    if vec.contains(&prev) {
                        return false;
                    }
                }
                None => continue
            }
        }
    }

    true
}

fn attempt_to_fix(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Option<Vec<i32>> {
    for (i, n) in update.iter().enumerate() {
        for (j, &prev) in update[..i].iter().enumerate() {
            match rules.get(&n) {
                Some(vec) => {
                    if vec.contains(&prev) {
                        let mut attempted_fix = update.clone();
                        attempted_fix.swap(i, j);
                        return Some(attempted_fix);
                    }
                }
                None => continue
            }
        }
    }

    None
}

fn sum_middlemost(updates: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for update in updates.iter() {
        total += update[update.len() / 2];
    }
    total
}