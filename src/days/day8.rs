use std::collections::{HashMap, HashSet};

pub fn part1(input: Vec<String>) -> String
{
    let antennas = get_antennas(&input);
    let frequencies = get_frequencies(&input, &antennas);
    format!("{}", frequencies.len())
}

pub fn part2(input: Vec<String>) -> String
{
    let antennas = get_antennas(&input);
    let frequencies = get_frequencies_again(&input, &antennas);
    format!("{}", frequencies.len())
}

fn get_antennas(map: &Vec<String>) -> HashMap<char, Vec<(i64, i64)>>
{
    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas.entry(char).or_insert_with(Vec::new).push((x as i64, y as i64));
            }
        }
    }

    antennas
}

fn get_frequencies(map: &Vec<String>, antennas: &HashMap<char, Vec<(i64, i64)>>) -> HashSet<(i64, i64)>
{
    let mut frequencies: HashSet<(i64, i64)> = HashSet::new();
    let antenna_types: Vec<char> = antennas.keys().cloned().collect();

    for a_type in antenna_types.iter() {
        let places = antennas.get(a_type).unwrap();
        for (i, &place1) in places.iter().enumerate() {
            for &place2 in places.iter().skip(i + 1) {
                let next_pos = get_next_pos(place1, place2);
                if is_within(map, next_pos) { frequencies.insert(next_pos); }
                let next_pos = get_next_pos(place2, place1);
                if is_within(map, next_pos) { frequencies.insert(next_pos); }
            }
        }
    }

    frequencies
}

fn get_frequencies_again(map: &Vec<String>, antennas: &HashMap<char, Vec<(i64, i64)>>) -> HashSet<(i64, i64)>
{
    let mut frequencies: HashSet<(i64, i64)> = HashSet::new();
    let antenna_types: Vec<char> = antennas.keys().cloned().collect();

    for a_type in antenna_types.iter() {
        let places = antennas.get(a_type).unwrap();
        for (i, &place1) in places.iter().enumerate() {
            for &place2 in places.iter().skip(i + 1) {

                frequencies.insert(place1);
                frequencies.insert(place2);

                let mut next_pos = get_next_pos(place1, place2);
                let mut carry_pos = place2;
                while is_within(map, next_pos) {
                    frequencies.insert(next_pos);
                    let old_next_pos = next_pos;
                    next_pos = get_next_pos(carry_pos, next_pos);
                    carry_pos = old_next_pos;
                }

                let mut next_pos = get_next_pos(place2, place1);
                let mut carry_pos = place1;
                while is_within(map, next_pos) {
                    frequencies.insert(next_pos);
                    let old_next_pos = next_pos;
                    next_pos = get_next_pos(carry_pos, next_pos);
                    carry_pos = old_next_pos;
                }
            }
        }
    }

    frequencies
}

fn get_next_pos(place1: (i64, i64), place2: (i64, i64)) -> (i64, i64)
{
    let diff = (place2.0-place1.0, place2.1-place1.1);
    (place2.0+diff.0, place2.1+diff.1)
}

fn is_within(map: &Vec<String>, pos: (i64, i64)) -> bool
{
    pos.1 < map.len() as i64 && pos.1 >= 0 && pos.0 < map.first().unwrap().len() as i64 && pos.0 >= 0
}