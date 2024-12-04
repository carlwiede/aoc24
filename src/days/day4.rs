pub fn part1(input: Vec<String>) -> String {

    let mut occurrences = 0;
    let directions = vec![
        (-1, 0), (1, 0), (0, -1), (0, 1),
        (-1, -1), (-1, 1), (1, -1), (1, 1),
    ];

    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != 'X' { continue }
            for dir in directions.iter() {
                if gandalf(&input, char.to_string(), x as i32 + dir.0, y as i32 + dir.1, *dir) { occurrences += 1 }
            }
        }
    }

    format!("{}", occurrences)
}

pub fn part2(input: Vec<String>) -> String {

    let mut occurrences = 0;

    for y in 0..input.len()-2 {
        for x in 0..input.len()-2 {
            if voldemort(&input, x, y) { occurrences += 1 }
        }
    }

    format!("{}", occurrences)
}

// Gandalf sends a magic beam of fire in the given direction to check for XMAS

fn gandalf(matrix: &Vec<String>, current: String, x: i32, y: i32, dir: (i32, i32)) -> bool {

    if current == "XMAS" { return true }
    if !"XMAS".contains(&current) { return false }

    if let Some(line) = matrix.get(y as usize) {
        if let Some(c) = line.chars().nth(x as usize) {
            return gandalf(matrix, current.clone() + &c.to_string(), x + dir.0, y + dir.1, dir)
        }
    }

    false
}

// Voldemort applies death and decay to a 3x3 character grid and checks if that grid contains the X-MAS or not

fn voldemort(matrix: &Vec<String>, x: usize, y: usize) -> bool {
    let main_diagonal: String = (0..3).map(|i| matrix[y+i].chars().nth(x+i).unwrap()).collect();
    let anti_diagonal: String = (0..3).map(|i| matrix[y+i].chars().nth(x+2-i).unwrap()).collect();
    let reverse_main_diagonal = main_diagonal.chars().rev().collect::<String>();
    let reverse_anti_diagonal = anti_diagonal.chars().rev().collect::<String>();

    (main_diagonal == "MAS" && anti_diagonal == "MAS")
    || (main_diagonal == "MAS" && reverse_anti_diagonal == "MAS")
    || (anti_diagonal == "MAS" && reverse_main_diagonal == "MAS")
    || (reverse_anti_diagonal == "MAS" && reverse_main_diagonal == "MAS")
}