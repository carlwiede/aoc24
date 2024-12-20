use std::collections::HashMap;

type Point = (usize, usize);

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part1(input: Vec<String>) -> String {

    let (grid, end) = get_racecourse(&input);
    let path: HashMap<Point, usize> = mark_path(&grid, &end);
    let cheats: HashMap<usize, usize> = get_cheats(&grid, &path);
    let mut total = 0;

    for c in cheats {
        if c.0 >= 100 { total += c.1 }
    }

    format!("{}", total)
}

pub fn part2(_input: Vec<String>) -> String {
    // Your Part 2 solution logic
    "Part 2 not implemented yet".to_string()
}

fn get_racecourse(input: &Vec<String>) -> (Vec<Vec<char>>, Point)
{
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; input.len()]; input.len()];
    let mut end: Point = (0, 0);

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'E' { end = (y, x) }
            else if c == '#' { grid[y][x] = '#' }
        }
    }

    (grid, end)
}

fn mark_path(grid: &Vec<Vec<char>>, end: &Point) -> HashMap<Point, usize>
{
    let mut path: HashMap<Point, usize> = HashMap::new();
    path.insert(*end, 0);

    let mut ps_left: usize = 1;
    let mut to_visit: Vec<Point> = Vec::new();
    to_visit.push(*end);

    while let Some(p) = to_visit.pop() {
        for d in DIRECTIONS {
            let next: Point = ((p.0 as i32 + d.0) as usize, (p.1 as i32 + d.1) as usize);
            if !path.contains_key(&next) && grid[next.0][next.1] != '#' {
                path.insert(next, ps_left);
                ps_left += 1;
                to_visit.push(next);
                break;
            }
        }
    }

    path
}

fn get_cheats(grid: &Vec<Vec<char>>, path: &HashMap<Point, usize>) -> HashMap<usize, usize>
{
    let mut cheats: HashMap<usize, usize> = HashMap::new();

    for p in path {
        for d in DIRECTIONS {
            let next: Point = ((p.0.0 as i32 + d.0) as usize, (p.0.1 as i32 + d.1) as usize);
            if grid[next.0][next.1] == '#' {
                let next_again: Point = ((next.0 as i32 + d.0) as usize, (next.1 as i32 + d.1) as usize);
                if path.contains_key(&next_again) && path.get(&next_again).unwrap() < p.1 {
                    let saved = p.1 - path.get(&next_again).unwrap() - 2;
                    cheats.entry(saved).and_modify(|value| *value += 1).or_insert(1);
                }
            }
        }
    }

    cheats
}