use std::collections::HashMap;
use rand::random;

type Point = (isize, isize);

pub fn part1(input: Vec<String>) -> String
{
    let mut num_keypad: HashMap<char, Point> = HashMap::new();
    num_keypad.insert('A', (3, 2));
    num_keypad.insert('0', (3, 1));
    num_keypad.insert('1', (2, 0));
    num_keypad.insert('2', (2, 1));
    num_keypad.insert('3', (2, 2));
    num_keypad.insert('4', (1, 0));
    num_keypad.insert('5', (1, 1));
    num_keypad.insert('6', (1, 2));
    num_keypad.insert('7', (0, 0));
    num_keypad.insert('8', (0, 1));
    num_keypad.insert('9', (0, 2));

    let mut dir_keypad: HashMap<char, Point> = HashMap::new();
    dir_keypad.insert('A', (0, 2));
    dir_keypad.insert('^', (0, 1));
    dir_keypad.insert('>', (1, 2));
    dir_keypad.insert('v', (1, 1));
    dir_keypad.insert('<', (1, 0));

    let mut total = 0;

    for line in input {
        let mut min_len = isize::MAX;
        for _ in 0..1000 {
            let mut seq = get_sequence(&line, &num_keypad, (3, 2), (3, 0));
            seq = get_sequence(&seq, &dir_keypad, (0, 2), (0, 0));
            seq = get_sequence(&seq, &dir_keypad, (0, 2), (0, 0));
            if (seq.len() as isize) < min_len { min_len = seq.len() as isize }
        }
        total += min_len * get_numeric_part(&line);
    }

    format!("{}", total)
}

pub fn part2(input: Vec<String>) -> String
{
    let mut num_keypad: HashMap<char, Point> = HashMap::new();
    num_keypad.insert('A', (3, 2));
    num_keypad.insert('0', (3, 1));
    num_keypad.insert('1', (2, 0));
    num_keypad.insert('2', (2, 1));
    num_keypad.insert('3', (2, 2));
    num_keypad.insert('4', (1, 0));
    num_keypad.insert('5', (1, 1));
    num_keypad.insert('6', (1, 2));
    num_keypad.insert('7', (0, 0));
    num_keypad.insert('8', (0, 1));
    num_keypad.insert('9', (0, 2));

    let mut dir_keypad: HashMap<char, Point> = HashMap::new();
    dir_keypad.insert('A', (0, 2));
    dir_keypad.insert('^', (0, 1));
    dir_keypad.insert('>', (1, 2));
    dir_keypad.insert('v', (1, 1));
    dir_keypad.insert('<', (1, 0));

    let mut total = 0;

    for line in input {
        let mut min_len = isize::MAX;
        for i in 0..1 {
            println!("{}", i);
            let mut seq = get_sequence(&line, &num_keypad, (3, 2), (3, 0));
            for _ in 0..25 {
                seq = get_sequence(&seq, &dir_keypad, (0, 2), (0, 0));
            }
            if (seq.len() as isize) < min_len { min_len = seq.len() as isize }
        }
        total += min_len * get_numeric_part(&line);
    }

    format!("{}", total)
}

fn get_sequence(target: &String, keypad: &HashMap<char, Point>, start: Point, null: Point) -> String
{
    let mut cp = start;
    let mut seq = String::new();

    for c in target.chars() {
        let pt = keypad.get(&c).unwrap();

        while cp != *pt {
            let diff = (pt.0 - cp.0, pt.1 - cp.1);

            let go: bool = random();
            if diff.0 > 0 && go {
                for _ in 0..diff.0 {
                    if (cp.0 + 1, cp.1) == null { break }
                    seq.push('v');
                    cp = (cp.0 + 1, cp.1);
                }
            }

            let go: bool = random();
            if diff.1 < 0 && go{
                for _ in 0..diff.1.abs() {
                    if (cp.0, cp.1 - 1) == null { break }
                    seq.push('<');
                    cp = (cp.0, cp.1 - 1);
                    break;
                }
            }

            let go: bool = random();
            if diff.1 > 0 && go{
                for _ in 0..diff.1  {
                    seq.push('>');
                    cp = (cp.0, cp.1 + 1);
                }
            }

            let go: bool = random();
            if diff.0 < 0 && go{
                for _ in 0..diff.0.abs() {
                    if (cp.0 - 1, cp.1) == null { break }
                    seq.push('^');
                    cp = (cp.0 - 1, cp.1);
                }
            }
        }

        seq.push('A');
    }

    seq
}

fn get_numeric_part(sequence: &String) -> isize
{
    sequence[..sequence.len()-1].parse().unwrap()
}
