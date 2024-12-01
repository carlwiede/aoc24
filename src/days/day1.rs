pub fn part1(input: Vec<String>) -> String {

    let mut total_diff = 0;
    let mut left_vals: Vec<i32> = vec![];
    let mut right_vals: Vec<i32> = vec![];

    for s in input.iter() {
        let vals: Vec<&str> = s.split_whitespace().collect();
        left_vals.push(vals.first().unwrap().parse::<i32>().unwrap());
        right_vals.push(vals.last().unwrap().parse::<i32>().unwrap());
    }

    for _ in input.iter() {
        if let (Some (&min_left), Some(&min_right)) = (left_vals.iter().min(), right_vals.iter().min()) {
            total_diff += (min_left - min_right).abs();

            if let Some(min_left_index) = left_vals.iter().position(|&x| x == min_left) {
                left_vals.remove(min_left_index);
            }
            if let Some(min_right_index) = right_vals.iter().position(|&x| x == min_right) {
                right_vals.remove(min_right_index);
            }
        }
    }

    format!("{}", total_diff)
}

use std::collections::HashMap;

pub fn part2(input: Vec<String>) -> String {
    
    let mut similarity_score = 0;
    let mut left_vals: Vec<i32> = vec![];
    let mut right_vals: HashMap<i32, i32> = HashMap::new();

    for s in input.iter() {
        let vals: Vec<&str> = s.split_whitespace().collect();
        left_vals.push(vals.first().unwrap().parse::<i32>().unwrap());
        *right_vals.entry(vals.last().unwrap().parse::<i32>().unwrap()).or_insert(0) += 1;
    }

    while let Some(val) = left_vals.pop() {
        similarity_score += val * right_vals.get(&val).unwrap_or(&0);
    }

    format!("{}", similarity_score)
}
