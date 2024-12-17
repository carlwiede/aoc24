pub fn part1(input: Vec<String>) -> String
{
    let (mut reg_a, mut reg_b, mut reg_c, ops) = read_program(&input);

    let mut out: Vec<i64> = Vec::new();
    let mut insp: i64 = 0;

    while let Some(op) = ops.get(insp as usize) {
        let operand: i64;
        if let Some(next) = ops.get((insp + 1) as usize) { operand = *next } else { break }

        match op {
            0 => { reg_a = reg_a / (2_i64.pow(get_combo_operand(operand, &reg_a, &reg_b, &reg_c) as u32)) },
            1 => { reg_b = reg_b ^ operand },
            2 => { reg_b = get_combo_operand(operand, &reg_a, &reg_b, &reg_c) % 8 },
            3 => { if reg_a != 0 { insp = operand; continue; }}
            4 => { reg_b = reg_b ^ reg_c },
            5 => { out.push(get_combo_operand(operand, &reg_a, &reg_b, &reg_c) % 8) },
            6 => { reg_b = reg_a / (2_i64.pow(get_combo_operand(operand, &reg_a, &reg_b, &reg_c) as u32)) },
            7 => { reg_c = reg_a / (2_i64.pow(get_combo_operand(operand, &reg_a, &reg_b, &reg_c) as u32)) },
            _ => panic!("op not implemented!")
        }

        insp += 2;
    }

    format!("{}", out.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","))
}

// Program specific, sorry
pub fn part2(input: Vec<String>) -> String
{
    let (_, _, _, mut ops) = read_program(&input);
    ops.reverse();

    let min_a = swag(0, &ops, &mut 0);
    
    format!("{}", min_a)
}

fn swag(i: i64, ops: &Vec<i64>, a: &mut i64) -> i64
{
    if i as usize >= ops.len() { return *a }
    let op = ops.get(i as usize).unwrap();
    *a <<= 3;
    for j in 0..8 {
        let b: i64 = j ^ 1;
        let c: i64 = (*a + j) >> b;
        if ((b ^ c) ^ 6) % 8 == *op {
            let mut new_a = a.clone() + j;
            let min_a = swag(i+1, ops, &mut new_a);
            if min_a != 0 { return min_a }
        }
    }
    0
}

fn read_program(input: &Vec<String>) -> (i64, i64, i64, Vec<i64>)
{
    let mut reg_a: i64 = 0;
    let mut reg_b: i64 = 0;
    let mut reg_c: i64 = 0;
    let mut ops: Vec<i64> = Vec::new();

    let mut is_program: bool = false;
    for line in input {
        if line.is_empty() {
            is_program = true;
            continue;
        }

        if !is_program {
            if line.contains("A") {
                reg_a = line.split_whitespace().last().unwrap().parse().unwrap();
            } else if line.contains("B") {
                reg_b = line.split_whitespace().last().unwrap().parse().unwrap();
            } else {
                reg_c = line.split_whitespace().last().unwrap().parse().unwrap();
            }
        } else {
            ops = line.split_whitespace().last().unwrap()
                      .split(",").filter_map(|num| num.parse::<i64>().ok())
                      .collect();
        }
    }

    (reg_a, reg_b, reg_c, ops)
}

fn get_combo_operand(operand: i64, reg_a: &i64, reg_b: &i64, reg_c: &i64) -> i64
{
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => *reg_a,
        5 => *reg_b,
        6 => *reg_c,
        _ => panic!("operand not valid!"),
    }
}