use std::collections::{BinaryHeap, HashSet, HashMap};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    y: i64,
    x: i64,
    dir: Direction,
    val: i64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct StateTwo {
    y: i64,
    x: i64,
    dir: Direction,
    val: i64,
    visited: HashSet<(i64, i64)>
}

impl Ord for StateTwo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for StateTwo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: Vec<String>) -> String
{
    let (grid, gamer) = parse_input(&input);
    let mut to_visit: BinaryHeap<State> = BinaryHeap::new();
    to_visit.push(State { y: gamer.0, x: gamer.1, dir: gamer.2, val: 0 });

    let mut end_val = 0;

    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    while let Some(state) = to_visit.pop() {
        if grid[state.y as usize][state.x as usize] == '#' { continue }
        if grid[state.y as usize][state.x as usize] == 'E' {
            end_val = state.val;
            break;
        }
        if visited.contains(&(state.y, state.x)) { continue }
        visited.insert((state.y, state.x));

        match state.dir {
            Direction::UP => {
                to_visit.push(State { y: state.y, x: state.x - 1, dir: Direction::LEFT, val: state.val + 1001 });
                to_visit.push(State { y: state.y - 1, x: state.x, dir: Direction::UP, val: state.val + 1 });
                to_visit.push(State { y: state.y, x: state.x + 1, dir: Direction::RIGHT, val: state.val + 1001 });
            }
            Direction::RIGHT => {
                to_visit.push(State { y: state.y - 1, x: state.x, dir: Direction::UP, val: state.val + 1001 });
                to_visit.push(State { y: state.y, x: state.x + 1, dir: Direction::RIGHT, val: state.val + 1 });
                to_visit.push(State { y: state.y + 1, x: state.x, dir: Direction::DOWN, val: state.val + 1001 });
            }
            Direction::DOWN => {
                to_visit.push(State { y: state.y, x: state.x + 1, dir: Direction::RIGHT, val: state.val + 1001 });
                to_visit.push(State { y: state.y + 1, x: state.x, dir: Direction::DOWN, val: state.val + 1 });
                to_visit.push(State { y: state.y, x: state.x - 1, dir: Direction::LEFT, val: state.val + 1001 });
            }
            Direction::LEFT => {
                to_visit.push(State { y: state.y + 1, x: state.x, dir: Direction::DOWN, val: state.val + 1001 });
                to_visit.push(State { y: state.y, x: state.x - 1, dir: Direction::LEFT, val: state.val + 1 });
                to_visit.push(State { y: state.y - 1, x: state.x, dir: Direction::UP, val: state.val + 1001 });
            }
        }
    }

    format!("{}", end_val)
}

pub fn part2(input: Vec<String>) -> String
{
    let (grid, gamer) = parse_input(&input);
    let mut to_visit: BinaryHeap<StateTwo> = BinaryHeap::new();
    to_visit.push(StateTwo { y: gamer.0, x: gamer.1, dir: gamer.2, val: 0, visited: HashSet::new() });

    let mut global_seen: HashMap<(i64, i64, Direction), i64> = HashMap::new();
    let mut best_visited: HashSet<(i64, i64)> = HashSet::new();
    let best_val = part1(input).parse().unwrap();

    while let Some(mut state) = to_visit.pop() {
        if grid[state.y as usize][state.x as usize] == '#' { continue }
        if state.val > best_val { continue }

        if let Some(seen) = global_seen.get(&(state.y, state.x, state.dir)) {
            if *seen < state.val {
                continue;
            }
        }

        if state.visited.contains(&(state.y, state.x)) { continue }

        state.visited.insert((state.y, state.x));
        global_seen.insert((state.y, state.x, state.dir), state.val);

        if grid[state.y as usize][state.x as usize] == 'E' {
            best_visited = best_visited.union(&state.visited).cloned().collect();
            continue;
        }

        match state.dir {
            Direction::UP => {
                to_visit.push(StateTwo { y: state.y, x: state.x - 1, dir: Direction::LEFT, val: state.val + 1001, visited: state.visited.clone() });
                to_visit.push(StateTwo { y: state.y - 1, x: state.x, dir: Direction::UP, val: state.val + 1, visited: state.visited.clone() });
                to_visit.push(StateTwo { y: state.y, x: state.x + 1, dir: Direction::RIGHT, val: state.val + 1001, visited: state.visited.clone() });
            }
            Direction::RIGHT => {
                to_visit.push(StateTwo { y: state.y - 1, x: state.x, dir: Direction::UP, val: state.val + 1001, visited: state.visited.clone() });
                to_visit.push(StateTwo { y: state.y, x: state.x + 1, dir: Direction::RIGHT, val: state.val + 1, visited: state.visited.clone() });
                to_visit.push(StateTwo { y: state.y + 1, x: state.x, dir: Direction::DOWN, val: state.val + 1001, visited: state.visited.clone() });
            }
            Direction::DOWN => {
                to_visit.push(StateTwo { y: state.y, x: state.x + 1, dir: Direction::RIGHT, val: state.val + 1001, visited: state.visited.clone() });
                to_visit.push(StateTwo { y: state.y + 1, x: state.x, dir: Direction::DOWN, val: state.val + 1, visited: state.visited.clone() });
                to_visit.push(StateTwo { y: state.y, x: state.x - 1, dir: Direction::LEFT, val: state.val + 1001, visited: state.visited.clone() });
            }
            Direction::LEFT => {
                to_visit.push(StateTwo { y: state.y + 1, x: state.x, dir: Direction::DOWN, val: state.val + 1001, visited: state.visited.clone() });
                to_visit.push(StateTwo { y: state.y, x: state.x - 1, dir: Direction::LEFT, val: state.val + 1, visited: state.visited.clone() });
                to_visit.push(StateTwo { y: state.y - 1, x: state.x, dir: Direction::UP, val: state.val + 1001, visited: state.visited.clone() });
            }
        }
    }

    format!("{}", best_visited.len())
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<char>>, (i64, i64, Direction))
{
    let mut grid: Vec<Vec<char>> = vec![];
    let mut gamer: (i64, i64, Direction) = (0, 0, Direction::RIGHT);

    for (y, line) in input.iter().enumerate() {
        grid.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            grid[y].push(c);
            if c == 'S' {
                gamer = (y as i64, x as i64, Direction::RIGHT);
            }
        }
    }

    (grid, gamer)
}