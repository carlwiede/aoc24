use std::collections::{HashMap, HashSet};

pub fn part1(input: Vec<String>) -> String
{
    let mut accounted: HashSet<(usize, usize)> = HashSet::new();
    let mut areas: Vec<(usize, usize)> = Vec::new();

    get_areas(&input, &mut accounted, &mut areas);

    let price: usize = areas.iter()
                            .map(|(area, perimeter)| area * perimeter)
                            .sum();
    format!("{}", price)
}

pub fn part2(input: Vec<String>) -> String
{
    let mut accounted: HashSet<(usize, usize)> = HashSet::new();
    let mut areas: Vec<(usize, usize)> = Vec::new();

    get_areas_two(&input, &mut accounted, &mut areas);

    let price: usize = areas.iter()
                            .map(|(area, perimeter)| area * perimeter)
                            .sum();
    format!("{}", price)
}

fn get_areas(map: &Vec<String>, accounted: &mut HashSet<(usize, usize)>, areas: &mut Vec<(usize, usize)>)
{
    for (y, line) in map.iter().enumerate() {
        for (x, _plant) in line.chars().enumerate() {
            if accounted.contains(&(x, y)) { continue }
            areas.push(get_region(map, accounted, x, y));
        }
    }
}

fn get_region(map: &Vec<String>, accounted: &mut HashSet<(usize, usize)>, x: usize, y: usize) -> (usize, usize)
{
    let region_char = map.get(y).unwrap().chars().nth(x).unwrap();
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    let mut area = 0;
    let mut perimeter = 0;
    to_visit.push((x, y));

    while !to_visit.is_empty() {
        let plant_pos = to_visit.pop().unwrap();
        if accounted.contains(&plant_pos) {
            continue;
        } else {
            accounted.insert(plant_pos);
            perimeter += 4;
            area += 1;
        }

        for n in [-1, 1].iter() {
            if let Some(line) = map.get((plant_pos.1 as i64 + n) as usize) {
                let plant = line.chars().nth(plant_pos.0).unwrap();
                if plant == region_char {
                    perimeter -= 1;
                    to_visit.push((plant_pos.0, (plant_pos.1 as i64 + n) as usize));
                }
            }

            if let Some(plant) = map.get(plant_pos.1).unwrap().chars().nth((plant_pos.0 as i64 + n) as usize) {
                if plant == region_char {
                    perimeter -= 1;
                    to_visit.push(((plant_pos.0 as i64 + n) as usize, plant_pos.1));
                }
            }
        }
    }

    (area, perimeter)
}

// Big brain for part 2:
// Create vertices around each plant
// 1 or 3 vertices = a side
// 2 or 4 vertices = not a side
fn get_areas_two(map: &Vec<String>, accounted: &mut HashSet<(usize, usize)>, areas: &mut Vec<(usize, usize)>)
{
    for (y, line) in map.iter().enumerate() {
        for (x, _plant) in line.chars().enumerate() {
            if accounted.contains(&(x, y)) { continue }
            areas.push(get_region_two(map, accounted, x, y));
        }
    }
}

fn get_region_two(map: &Vec<String>, accounted: &mut HashSet<(usize, usize)>, x: usize, y: usize) -> (usize, usize)
{
    let region_char = map.get(y).unwrap().chars().nth(x).unwrap();
    let mut vertices: HashMap<(usize, usize), u8> = HashMap::new();
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    let mut area = 0;
    to_visit.push((x, y));

    while !to_visit.is_empty() {
        let plant_pos = to_visit.pop().unwrap();
        if accounted.contains(&plant_pos) {
            continue;
        } else {
            accounted.insert(plant_pos);
            area += 1;
        }

        *vertices.entry(plant_pos).or_insert(0) += 1;
        *vertices.entry((plant_pos.0 + 1, plant_pos.1)).or_insert(0) += 1;
        *vertices.entry((plant_pos.0, plant_pos.1 + 1)).or_insert(0) += 1;
        *vertices.entry((plant_pos.0 + 1, plant_pos.1 + 1)).or_insert(0) += 1;

        for n in [-1, 1].iter() {
            if let Some(line) = map.get((plant_pos.1 as i64 + n) as usize) {
                let plant = line.chars().nth(plant_pos.0).unwrap();
                if plant == region_char {
                    to_visit.push((plant_pos.0, (plant_pos.1 as i64 + n) as usize));
                }
            }

            if let Some(plant) = map.get(plant_pos.1).unwrap().chars().nth((plant_pos.0 as i64 + n) as usize) {
                if plant == region_char {
                    to_visit.push(((plant_pos.0 as i64 + n) as usize, plant_pos.1));
                }
            }
        }
    }

    let mut sides = vertices.iter()
                                   .filter(|&(_, value)| value % 2 != 0)
                                   .count();

    // edge case, please look away
    for v  in  vertices.iter()
                                               .filter(|&(_, value)| *value == 2) {
        if let Some(line) = map.get(v.0.1) {
            if let Some(plant) = line.chars().nth(v.0.0) {
                if let Some(plant_left) = line.chars().nth((v.0.0 as i64 - 1) as usize) {
                    if let Some(above_line) = map.get((v.0.1 as i64 - 1) as usize) {
                        if let Some(plant_up) = above_line.chars().nth(v.0.0) {
                            if let Some(plant_up_left) = above_line.chars().nth((v.0.0 as i64 - 1) as usize) {
                                if plant == region_char
                                   && plant_up_left == region_char
                                   && plant_left != region_char
                                   && plant_up != region_char {
                                    sides += 2;
                                } else if plant_left == region_char
                                          && plant_up == region_char
                                          && plant != region_char
                                          && plant_up_left != region_char {
                                    sides += 2;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (area, sides)
}
