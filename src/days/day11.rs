use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> String
{
    let blinks = 25;
    let mut stones = input.first().unwrap().clone().split_whitespace().map(String::from).collect();

    for _ in 0..blinks {
        stones = blink(&stones);
    }

    format!("{}", stones.len())
}

pub fn part2(input: Vec<String>) -> String
{
    let stones: Vec<usize> = input.first().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();

    let blinks = 75;
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut total = 0;

    for s in stones.iter() {
        total += blink_cached(*s, &mut cache, blinks)
    }

    format!("{}", total)
}

fn blink(stones: &Vec<String>) -> Vec<String>
{
    let mut new_stones: Vec<String> = Vec::new();

    for s in stones.iter() {
        if s == "0" { new_stones.push(String::from("1")); }
        else if s.len() % 2 == 0 {
            let (first, second) = s.split_at(s.len() / 2);
            new_stones.push(String::from(first));
            new_stones.push(second.parse::<usize>().unwrap().to_string());
        }
        else { new_stones.push((s.parse::<usize>().unwrap() * 2024).to_string()); }
    }

    new_stones
}

fn blink_cached(stone: usize, cache: &mut HashMap<(usize, usize), usize>, blinks: usize) -> usize
{
    if blinks == 0 { return 1 }
    if let Some(cached_val) = cache.get(&(blinks, stone)) {
        return *cached_val;
    }

    let stone_val: usize;
    if stone == 0 {
        stone_val = blink_cached(1, cache, blinks-1);
    } else if stone.to_string().len() % 2 == 0 {
        let s_str = stone.to_string();
        let (first, second) = s_str.split_at(s_str.len() / 2);
        let (left, right) = (first.parse::<usize>().unwrap(), second.parse::<usize>().unwrap());
        stone_val = blink_cached(left, cache, blinks-1) + blink_cached(right, cache, blinks-1);
    } else {
        stone_val = blink_cached(stone * 2024, cache, blinks-1);
    }

    cache.insert((blinks, stone), stone_val);
    return stone_val
}
