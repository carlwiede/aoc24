use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> String
{
    let (available, desired) = get_towels(&input);
    let mut possible = 0;

    for d in desired {
        possible += is_possible(&available, d);
    }

    format!("{}", possible)
}

pub fn part2(input: Vec<String>) -> String
{
    let (available, desired) = get_towels(&input);
    let mut possible = 0;
    let mut cache: HashMap<String, i64> = HashMap::new();

    for d in desired {
        possible += is_possible_two(&available, d, &mut cache);
    }

    format!("{}", possible)
}

fn get_towels(input: &Vec<String>) -> (Vec<String>, Vec<String>)
{
    let mut is_desired = false;
    let mut available: Vec<String> = Vec::new();
    let mut desired: Vec<String> = Vec::new();

    for line in input {
        if line.is_empty() { is_desired = true; continue; }

        if is_desired {
            desired.push(line.clone());
        } else {
            let mut sb: String = String::new();
            for c in line.chars() {
                if c == ',' {
                    available.push(sb.clone());
                    sb.clear();
                }
                else if c != ' ' {
                    sb.push(c);
                }
            }
            available.push(sb);
        }
    }

    (available, desired)
}

fn is_possible(available: &Vec<String>, desired: String) -> i64
{
    if desired.is_empty() { return 1 }
    for a in available {
        if desired.starts_with(a) {
            let is_possible = is_possible(available, desired[a.len()..].to_string());
            if is_possible == 1 { return 1 }
        }
    }
    0
}

fn is_possible_two(available: &Vec<String>, desired: String, cache: &mut HashMap<String, i64>) -> i64
{
    if desired.is_empty() { return 1 }
    if cache.contains_key(&desired) {
        return *cache.get(&desired).unwrap();
    }
    let mut total_possible = 0;
    for a in available {
        if desired.starts_with(a) {
            total_possible += is_possible_two(available, desired[a.len()..].to_string(), cache);
        }
    }
    cache.insert(desired,total_possible);
    total_possible
}
