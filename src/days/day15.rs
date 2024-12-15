use std::collections::HashSet;

#[derive(Debug)]
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

pub fn part2(_input: Vec<String>) -> String
{
    "sadness".to_string()
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
