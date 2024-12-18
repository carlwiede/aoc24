use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);

pub fn part1(input: Vec<String>) -> String
{
    const SIZE: usize = 71;
    const N_BYTES: usize = 1024;

    let memory = get_memory(SIZE, N_BYTES, &input);
    format!("{}", get_path(&memory, SIZE).unwrap().len()-1)
}

pub fn part2(input: Vec<String>) -> String
{
    const SIZE: usize = 71;
    const N_BYTES: usize = 1024;
    let mut memory = get_memory(SIZE, N_BYTES, &input);

    let mut next_byte = N_BYTES;
    add_byte(&mut memory, input[next_byte].clone());

    while let Some(_) = get_path(&memory, SIZE) {
        next_byte += 1;
        add_byte(&mut memory, input[next_byte].clone());
    }

    format!("{}", input[next_byte])
}

fn get_memory(size: usize, n_bytes: usize, bytes: &Vec<String>) -> Vec<Vec<u8>>
{
    let mut memory: Vec<Vec<u8>> = vec![vec![0; size]; size];
    let mut n = 0;

    for b in bytes {
        if n == n_bytes { break }

        let (xs, ys) = b.split_once(",").unwrap();
        let x: usize = xs.parse().unwrap();
        let y: usize = ys.parse().unwrap();
        memory[y][x] = 1;

        n += 1;
    }

    memory
}

fn add_byte(memory: &mut Vec<Vec<u8>>, byte: String)
{
    let (xs, ys) = byte.split_once(",").unwrap();
    let x: usize = xs.parse().unwrap();
    let y: usize = ys.parse().unwrap();
    memory[y][x] = 1;
}

// BFS
fn get_path(grid: &Vec<Vec<u8>>, size: usize) -> Option<Vec<Point>>
{
    let dirs = [(0, 1), (-1, 0), (1, 0), (0, -1)];

    let mut queue: VecDeque<(Point, Vec<Point>)> = VecDeque::new();
    queue.push_back(((0, 0), vec![(0, 0)]));

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert((0, 0));

    while let Some(((y, x), path)) = queue.pop_front() {
        if (y, x) == (size - 1, size - 1) {
            return Some(path);
        }

        for (dx, dy) in dirs.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if ny >= 0 && ny < size as isize && nx >= 0 && nx < size as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[ny][nx] == 0 && !visited.contains(&(ny, nx)) {
                    let mut new_path = path.clone();
                    new_path.push((ny, nx));
                    queue.push_back(((ny, nx), new_path));
                    visited.insert((ny, nx));
                }
            }
        }
    }

    None
}
