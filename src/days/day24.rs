use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> String {

    let (mut values, mut to_calculate) = read_input(&input);

    let mut z_num = 0;

    while let Some(s) = to_calculate.pop() {
        let (r1, op, r2, d) = get_parts(&s);

        if !values.contains_key(&r1) || !values.contains_key(&r2) {
            to_calculate.insert(0, s);
            continue;
        }

        values.insert(d, match op.as_str() {
            "AND" => values.get(&r1).unwrap() & values.get(&r2).unwrap(),
            "XOR" => values.get(&r1).unwrap() ^ values.get(&r2).unwrap(),
            "OR" => values.get(&r1).unwrap() | values.get(&r2).unwrap(),
            _ => panic!("Unknown op")
        });
    }

    let mut val_vec: Vec<(String, bool)> = values.into_iter().collect();
    val_vec.sort();
    val_vec.reverse();
    for (i, v) in val_vec.iter().enumerate() {
        if v.0 == "z00" {
            val_vec = val_vec[..i+1].to_vec();
            break;
        }
    }

    for (i, v) in val_vec.iter().enumerate() {
        let val: i64 = match v.1 {
            false => 0,
            true => 1
        } << (val_vec.len() - i - 1);
        z_num += val;
    }

    format!("{}", z_num)
}

pub fn part2(_input: Vec<String>) -> String {
    // Your Part 2 solution logic
    "Part 2 not implemented yet".to_string()
}

fn read_input(input: &Vec<String>) -> (HashMap<String, bool>, Vec<String>)
{
    let mut values: HashMap<String, bool> = HashMap::new();
    let mut to_calculate: Vec<String> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        if line.is_empty() {
            to_calculate = input[i+1..].to_vec();
            break;
        }
        let (reg, val) = line.split_once(": ").unwrap();
        values.insert(reg.to_string(), match val {
            "0" => false,
            "1" => true,
            _ => panic!("Could not parse bool pppp")
        });
    }

    (values, to_calculate)
}

fn get_parts(line: &String) -> (String, String, String, String)
{
    let parts: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
    (parts[0].clone(), parts[1].clone(), parts[2].clone(), parts[4].clone())
}
