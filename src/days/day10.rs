use std::collections::HashSet;

pub fn part1(input: Vec<String>) -> String
{
    let mut total_score = 0;
    let trailheads: Vec<(i64, i64)> = get_trailheads(&input);
    for t in trailheads.iter() {
        let mut visited_tops: HashSet<(i64, i64)> = HashSet::new();
        get_score(&input, &t, -1, &mut visited_tops);
        total_score += visited_tops.len();
    }
    format!("{}", total_score)
}

pub fn part2(input: Vec<String>) -> String
{
    let mut total_score = 0;
    let trailheads: Vec<(i64, i64)> = get_trailheads(&input);
    for t in trailheads.iter() {
        total_score += get_score_two(&input, &t, -1);
    }
    format!("{}", total_score)
}

fn get_trailheads(map: &Vec<String>) -> Vec<(i64, i64)>
{
    let mut trailheads: Vec<(i64, i64)> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.chars().enumerate() {
            if height.to_string().parse::<i64>().unwrap() == 0 { trailheads.push((x as i64, y as i64)) }
        }
    }
    trailheads
}

fn get_score(map: &Vec<String>, trailpos: &(i64, i64), previous: i64, visited_tops: &mut HashSet<(i64, i64)>)
{
    if let Some(line) = map.get(trailpos.1 as usize) {
        if let Some(char) = line.chars().nth(trailpos.0 as usize) {
            let height = char.to_string().parse::<i64>().unwrap();

            if previous != height - 1 { return }

            if height == 9 {
                visited_tops.insert((trailpos.0, trailpos.1));
                return;
            }

            get_score(map, &(trailpos.0, trailpos.1 - 1), height, visited_tops);
            get_score(map, &(trailpos.0, trailpos.1 + 1), height, visited_tops);
            get_score(map, &(trailpos.0 - 1, trailpos.1), height, visited_tops);
            get_score(map, &(trailpos.0 + 1, trailpos.1), height, visited_tops);
        }
    }
}

fn get_score_two(map: &Vec<String>, trailpos: &(i64, i64), previous: i64) -> i64
{
    if let Some(line) = map.get(trailpos.1 as usize) {
        if let Some(char) = line.chars().nth(trailpos.0 as usize) {
            let height = char.to_string().parse::<i64>().unwrap();

            if previous != height - 1 { return 0 }

            if height == 9 { return 1 }

            return get_score_two(map, &(trailpos.0, trailpos.1 - 1), height)
                 + get_score_two(map, &(trailpos.0, trailpos.1 + 1), height)
                 + get_score_two(map, &(trailpos.0 - 1, trailpos.1), height)
                 + get_score_two(map, &(trailpos.0 + 1, trailpos.1), height)
        }
    }

    0
}