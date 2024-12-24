use std::collections::{HashMap, HashSet};

pub fn part1(input: Vec<String>) -> String
{
    let computers = get_computers(&input);

    let mut threes_with_t: HashSet<(String, String, String)> = HashSet::new();

    for c in computers.keys().filter(|k| k.starts_with("t")) {
        let connections: Vec<&String> = computers.get(c).unwrap().iter().collect();
        for i in 0..connections.len()-1 {
            for j in i+1..connections.len() {
                if computers.get(connections[i]).unwrap().contains(connections[j]) {
                    if !threes_with_t.contains(&(connections[i].clone(), c.clone(), connections[j].clone()))
                        && !threes_with_t.contains(&(connections[i].clone(), connections[j].clone(), c.clone()))
                        && !threes_with_t.contains(&(connections[j].clone(), connections[i].clone(), c.clone()))
                        && !threes_with_t.contains(&(connections[j].clone(), c.clone(), connections[i].clone()))
                        && !threes_with_t.contains(&(c.clone(), connections[j].clone(), connections[i].clone())) {
                        threes_with_t.insert((c.clone(), connections[i].clone(), connections[j].clone()));
                    }
                }
            }
        }
    }

    format!("{}", threes_with_t.len())
}

pub fn part2(input: Vec<String>) -> String
{
    let computers = get_computers(&input);

    let mut largest_network: HashSet<String> = HashSet::new();

    let mut largest_vec: Vec<String> = largest_network.into_iter().collect();
    largest_vec.sort();
    format!("{}", largest_vec.join(","))
}

fn get_computers(input: &Vec<String>) -> HashMap<String, HashSet<String>>
{
    let mut computers: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input {
        let (first, second) = line.split_once("-").unwrap();
        computers.entry(first.to_string()).or_insert_with(HashSet::new).insert(second.to_string());
        computers.entry(second.to_string()).or_insert_with(HashSet::new).insert(first.to_string());
    }

    computers
}