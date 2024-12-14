struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

pub fn part1(input: Vec<String>) -> String
{
    let mut map = get_initial_map(&input);
    for _ in 0..100 {
        map = walk_robots(map);
    }
    let safety_factor = get_safety_factor(map);
    format!("{}", safety_factor)
}

pub fn part2(input: Vec<String>) -> String
{
    let mut map = get_initial_map(&input);
    let mut i = 0;
    loop {
        i += 1;
        map = walk_robots(map);
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell = '.';
            }
        }
        for r in &map {
            grid[r.pos.1 as usize][r.pos.0 as usize] = '@';
        }
        if is_frame_found(&grid) { break }
    }

    format!("{}", i)
}

fn get_initial_map(input: &Vec<String>) -> Vec<Robot>
{
    let mut map: Vec<Robot> = Vec::new();

    for line in input.iter() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let (raw_p, raw_v) = (split[0], split[1]);
        let p: Vec<&str> = raw_p[2..].split(",").collect();
        let px: i64 = p[0].parse().unwrap();
        let py: i64 = p[1].parse().unwrap();
        let v: Vec<&str> = raw_v[2..].split(",").collect();
        let dx: i64 = v[0].parse().unwrap();
        let dy: i64 = v[1].parse().unwrap();
        map.push(Robot {pos: (px, py), vel: (dx, dy)});
    }

    map
}

fn walk_robots(map: Vec<Robot>) -> Vec<Robot>
{
    let mut new_map: Vec<Robot> = Vec::new();

    for r in map {
        let new_robot: Robot = Robot {
            pos: ((r.pos.0 + r.vel.0 + WIDTH) % WIDTH, (r.pos.1 + r.vel.1 + HEIGHT) % HEIGHT),
            vel: r.vel
        };
        new_map.push(new_robot);
    }

    new_map
}

fn get_safety_factor(map: Vec<Robot>) -> i64
{
    let mut quadrants: Vec<i64> = vec![0; 4];

    for r in map {
        if r.pos.0 < WIDTH / 2 && r.pos.1 < HEIGHT / 2 {
            quadrants[0] += 1;
        }
        else if r.pos.0 > WIDTH / 2 && r.pos.1 < HEIGHT / 2 {
            quadrants[1] += 1;
        }
        else if r.pos.0 > WIDTH / 2 && r.pos.1 > HEIGHT / 2 {
            quadrants[2] += 1;
        }
        else if r.pos.0 < WIDTH / 2 && r.pos.1 > HEIGHT / 2 {
            quadrants[3] += 1;
        }
    }

    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

fn is_frame_found(grid: &[Vec<char>]) -> bool
{
    for row in grid {
        let mut count = 0;

        for &cell in row {
            if cell == '@' {
                count += 1;
                if count == 16 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }

    false
}
