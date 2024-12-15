use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    pub fn val(&self) -> (i8, i8) {
        match self {
            Direction::UP => (-1, 0),
            Direction::RIGHT => (0, 1),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
        }
    }
}

pub fn part1(input: Vec<String>) -> String
{
    let (mut lanternfish, mut boxes, walls, moves) = parse_input(&input);

    for d in moves {
        try_move(&mut lanternfish, &mut boxes, &walls, d);
    }

    let gps_sum = sum_boxes(boxes);
    format!("{}", gps_sum)
}

pub fn part2(input: Vec<String>) -> String
{
    let (mut grid, mut lanternfish, moves) = parse_input_two(&input);

    for d in moves {
        try_move_two(&mut grid, &mut lanternfish, d)
    }

    let gps_sum = sum_boxes_two(&grid);
    format!("{}", gps_sum)
}

fn parse_input(input: &Vec<String>) -> ((i64, i64), HashSet<(i64, i64)>, HashSet<(i64, i64)>, Vec<Direction>)
{
    let mut boxes: HashSet<(i64, i64)> = HashSet::new();
    let mut walls: HashSet<(i64, i64)> = HashSet::new();
    let mut lanternfish: (i64, i64) = (0, 0);
    let mut moves: Vec<Direction> = Vec::new();

    let mut is_direction = false;

    for (y, line) in input.iter().enumerate() {
        if line.is_empty() { is_direction = true; continue; }

        if !is_direction {
            for (x, c) in line.chars().enumerate() {
                if c == '#' { walls.insert((y as i64, x as i64)); }
                else if c == 'O' { boxes.insert((y as i64, x as i64)); }
                else if c == '@' { lanternfish = (y as i64, x as i64); }
            }
        } else {
            for d in line.chars() {
                match d {
                    '^' => moves.push(Direction::UP),
                    '>' => moves.push(Direction::RIGHT),
                    'v' => moves.push(Direction::DOWN),
                    '<' => moves.push(Direction::LEFT),
                    _ => panic!("invalid move found")
                }
            }
        }
    }

    (lanternfish, boxes, walls, moves)
}

fn try_move(lanternfish: &mut (i64, i64), boxes: &mut HashSet<(i64, i64)>, walls: &HashSet<(i64, i64)>, dir: Direction)
{
    let dv = dir.val();
    let want_to = (lanternfish.0 + dv.0 as i64, lanternfish.1 + dv.1 as i64);

    if walls.contains(&want_to) {
        return
    }

    if boxes.contains(&want_to) {
        if !box_move_chain(&want_to, boxes, walls, dir) {
            return
        }
    }

    *lanternfish = want_to;
}

fn box_move_chain(current: &(i64, i64), boxes: &mut HashSet<(i64, i64)>, walls: &HashSet<(i64, i64)>, dir: Direction) -> bool
{
    let dv = dir.val();
    let want_to = (current.0 + dv.0 as i64, current.1 + dv.1 as i64);

    if walls.contains(&want_to) {
        return false;
    }

    if boxes.contains(&want_to) {
        if !box_move_chain(&want_to, boxes, walls, dir) {
            return false;
        }
    }

    boxes.remove(current);
    boxes.insert(want_to);
    true
}

fn sum_boxes(boxes: HashSet<(i64, i64)>) -> i64
{
    let mut sum = 0;
    for b in boxes.iter() {
        sum += 100 * b.0 + b.1;
    }
    sum
}

fn parse_input_two(input: &Vec<String>) -> (Vec<Vec<char>>, (i64, i64), Vec<Direction>)
{
    let mut grid: Vec<Vec<char>> = vec![];
    let mut lanternfish: (i64, i64) = (0, 0);
    let mut moves: Vec<Direction> = Vec::new();

    let mut is_direction = false;

    for (y, line) in input.iter().enumerate() {
        if line.is_empty() { is_direction = true; continue; }

        if !is_direction {
            grid.push(Vec::new());
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    grid[y].push(c);
                    grid[y].push(c);
                }
                else if c == 'O' {
                    grid[y].push('[');
                    grid[y].push(']');
                }
                else if c == '@' {
                    lanternfish = (y as i64, x as i64 * 2);
                    grid[y].push('@');
                    grid[y].push('.');
                }
                else {
                    grid[y].push('.');
                    grid[y].push('.');
                }
            }
        } else {
            for d in line.chars() {
                match d {
                    '^' => moves.push(Direction::UP),
                    '>' => moves.push(Direction::RIGHT),
                    'v' => moves.push(Direction::DOWN),
                    '<' => moves.push(Direction::LEFT),
                    _ => panic!("invalid move found")
                }
            }
        }
    }

    (grid, lanternfish, moves)
}

fn try_move_two(grid: &mut Vec<Vec<char>>, lanternfish: &mut (i64, i64), dir: Direction)
{
    let dv = dir.val();
    let want_to = (lanternfish.0 + dv.0 as i64, lanternfish.1 + dv.1 as i64);

    if grid[want_to.0 as usize][want_to.1 as usize] == '#' {
        return
    }

    let mut visited: HashSet<(i64, i64, char)> = HashSet::new();
    visited.insert((lanternfish.0, lanternfish.1, '@'));

    if grid[want_to.0 as usize][want_to.1 as usize] == '['
       || grid[want_to.0 as usize][want_to.1 as usize] == ']' {

        if !can_box_move(&grid, want_to, &mut visited, dir) {
            return;
        }
    }

    move_visited(grid, visited, dir);
    *lanternfish = want_to;
}

fn can_box_move(grid: &Vec<Vec<char>>, current: (i64, i64), visited: &mut HashSet<(i64, i64, char)>, dir: Direction) -> bool
{
    let my_char = grid[current.0 as usize][current.1 as usize];
    if visited.contains(&(current.0, current.1, my_char)) {
        return true
    }
    visited.insert((current.0, current.1, my_char));

    let mut can_move_entire: bool = true;
    if (my_char == '[' && dir == Direction::UP) || (my_char == '[' && dir == Direction::DOWN) {
        can_move_entire = can_box_move(grid, (current.0, current.1 + 1), visited, dir);
    } else if (my_char == ']' && dir == Direction::UP) || (my_char == ']' && dir == Direction::DOWN) {
        can_move_entire = can_box_move(grid, (current.0, current.1 - 1), visited, dir);
    }

    let dv = dir.val();
    let want_to = (current.0 + dv.0 as i64, current.1 + dv.1 as i64);

    if grid[want_to.0 as usize][want_to.1 as usize] == '#' {
        return false
    }

    if grid[want_to.0 as usize][want_to.1 as usize] == '['
       || grid[want_to.0 as usize][want_to.1 as usize] == ']' {
        return can_move_entire && can_box_move(grid, want_to, visited, dir);
    }

    can_move_entire
}

fn move_visited(grid: &mut Vec<Vec<char>>, visited: HashSet<(i64, i64, char)>, dir: Direction)
{
    let mut new_grid = grid.clone();
    let dv = dir.val();

    for v in &visited {
        new_grid[v.0 as usize][v.1 as usize] = '.';
    }

    for v in visited {
        new_grid[(v.0 + dv.0 as i64) as usize][(v.1 + dv.1 as i64) as usize] = v.2;
    }

    *grid = new_grid;
}

fn sum_boxes_two(grid: &Vec<Vec<char>>) -> i64
{
    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '[' { sum += y * 100 + x }
        }
    }
    sum as i64
}
