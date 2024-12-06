use std::collections::HashMap;

const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1), (1, 0), (0, 1), (-1, 0),
];

pub fn part1(input: Vec<String>) -> String
{
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut guard = find_guard(&input);

    visit_current_position(&mut visited, &guard);

    while let Some(new_guard) = walk(&input, &guard) {
        guard = new_guard;
        visit_current_position(&mut visited, &guard);
    }

    format!("{}", visited.len())
}

pub fn part2(input: Vec<String>) -> String
{
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut guard = find_guard(&input);

    visit_current_position(&mut visited, &guard);

    while let Some(new_guard) = walk(&input, &guard) {
        guard = new_guard;
        visit_current_position(&mut visited, &guard);
    }

    let positions: Vec<(usize, usize)> = visited.keys().cloned().collect();

    let base_input = input.clone();

    let mut successful_attempts = 0;

    for position in positions {

        let mut input: Vec<String> = base_input.clone();
        let mut guard: ((usize, usize), usize) = find_guard(&input);

        if !place_obstruction(&mut input, position) {
            continue;
        }

        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

        visit_current_position(&mut visited, &guard);

        while let Some(new_guard) = walk(&input, &guard) {

            guard = new_guard;

            if current_position_is_visited(&visited, &guard) {
                successful_attempts += 1;
                break;
            }

            visit_current_position(&mut visited, &guard);
        }
    }

    format!("{}", successful_attempts)
}

fn find_guard(map: &Vec<String>) -> ((usize, usize), usize)
{
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '^' { return ((x, y), 0) }
        }
    }

    unreachable!()
}

fn walk(map: &Vec<String>, guard: &((usize, usize), usize)) -> Option<((usize, usize), usize)>
{
    if let Some(line) = map.get((guard.0.1 as i32 + DIRECTIONS[guard.1].1) as usize) {
        if let Some(char) = line.chars().nth((guard.0.0 as i32 + DIRECTIONS[guard.1].0) as usize) {
            if char == '#' || char == 'O' {
                return Some(((guard.0.0, guard.0.1), (guard.1 + 1) % 4));
            } else {
                return Some((((guard.0.0 as i32 + DIRECTIONS[guard.1].0) as usize,
                              (guard.0.1 as i32 + DIRECTIONS[guard.1].1) as usize),
                               guard.1))
            }
        }
    }

    None // End of map
}

fn visit_current_position(trail: &mut HashMap<(usize, usize), usize>, guard: &((usize, usize), usize))
{
    trail.entry(guard.0)
        .and_modify(|dir| *dir |= (2 as usize).pow(guard.1 as u32))
        .or_insert((2 as usize).pow(guard.1 as u32));
}

fn current_position_is_visited(trail: &HashMap<(usize, usize), usize>, guard: &((usize, usize), usize)) -> bool
{
    *trail.get(&guard.0).unwrap_or(&(0 as usize)) & (2 as usize).pow(guard.1 as u32) != 0
}

fn place_obstruction(map: &mut Vec<String>, pos: (usize, usize)) -> bool
{
    if let Some(line) = map.get_mut(pos.1) {
        if let Some(c) = line.chars().nth(pos.0) {
            if c == '^' {
                return false;
            }
            let mut chars: Vec<char> = line.chars().collect();
            chars[pos.0] = 'O';
            *line = chars.into_iter().collect();
        }
    }

    true
}