pub fn part1(input: Vec<String>) -> String
{
    let mut total = 0;
    let cost_a = 3;
    let cost_b = 1;

    let len = input.len();
    let mut i = 0;
    while i < len - 2 {
        let (mut best_a, mut best_b) = (0, 0);
        let mut min = std::i64::MAX;
        let part = &input[i..i+3];
        let (ax, ay, bx, by, tx, ty) = get_claw(part);

        // initiate brute force
        for a in 0..=tx / ax {
            for b in 0..=tx / bx {
                let x = a * ax + b * bx;
                let y = a * ay + b * by;

                if x == tx && y == ty {
                    let cost = a * cost_a + b * cost_b;
                    if cost < min {
                        min = cost;
                        best_a = a;
                        best_b = b;
                    }
                }
            }
        }

        if min < std::i64::MAX {
            total += best_a * cost_a + best_b * cost_b;
        }

        i += 4;
    }

    format!("{}", total)
}

pub fn part2(input: Vec<String>) -> String
{
    let mut total = 0;
    let cost_a = 3;
    let cost_b = 1;

    let len = input.len();
    let mut i = 0;
    while i < len - 2 {
        let part = &input[i..i+3];
        i += 4;
        let (ax, ay, bx, by, mut tx, mut ty) = get_claw(part);
        tx += 10000000000000;
        ty += 10000000000000;

        // let's be a bit smarter (quick maths)
        let na = ty * bx - tx * by;
        let da = ay * bx - ax * by;
        if na % da != 0 {
            continue;
        }

        let nb = ty * ax - tx * ay;
        let db = -da;
        if nb % db != 0 {
            continue;
        }

        total += na / da * cost_a + nb / db * cost_b;
    }

    format!("{}", total)
}

fn get_claw(desc: &[String]) -> (i64, i64, i64, i64, i64, i64)
{
    let a: Vec<&str> = desc[0].split_whitespace().collect::<Vec<&str>>()[2..].to_vec();
    let b: Vec<&str> = desc[1].split_whitespace().collect::<Vec<&str>>()[2..].to_vec();
    let t: Vec<&str> = desc[2].split_whitespace().collect::<Vec<&str>>()[1..].to_vec();

    let ax: i64 = a[0][..a[0].len()-1][2..].parse().unwrap();
    let ay: i64 = a[1][2..].parse().unwrap();
    let bx: i64 = b[0][..b[0].len()-1][2..].parse().unwrap();
    let by: i64 = b[1][2..].parse().unwrap();
    let tx: i64 = t[0][..t[0].len()-1][2..].parse().unwrap();
    let ty: i64 = t[1][2..].parse().unwrap();

    (ax, ay, bx, by, tx, ty)
}
