use itertools::iproduct;
use rayon::prelude::*;

pub fn part1(input: Vec<String>) -> String
{
    let mut total = 0;

    for line in input {
        let mut secret = line.parse().unwrap();
        for _ in 0..2000 {
            secret = get_next_secret(secret);
        }
        total += secret;
    }

    format!("{}", total)
}

pub fn part2(input: Vec<String>) -> String
{
    let mut change_windows: Vec<Vec<(isize, isize)>> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        let mut secret = line.parse().unwrap();
        let mut last_secret = secret;
        change_windows.push(Vec::new());
        for _ in 0..2000 {
            secret = get_next_secret(secret);
            change_windows[i].push((secret % 10, secret % 10 - last_secret % 10));
            last_secret = secret;
        }
    }

    let most_bananas = iproduct!(-9..=9, -9..=9, -9..=9, -9..=9)
        .par_bridge()
        .filter_map(|(x, y, z, w)| {
            let mut should_bail = false;
            for window_size in 2..=4 {
                for window in [x,y,z,w].windows(window_size) {
                    let sum: isize = window.iter().sum();
                    if sum < -9 || sum > 9 {
                        should_bail = true;
                        break;
                    }
                }
                if should_bail { break }
            }
            if should_bail { return None }

            let mut temporary_total = 0;

            for cw in &change_windows {
                let mut first_occurence = 0;
                for win in cw.windows(4) {
                    if [x, y, z, w] == [win[0].1, win[1].1, win[2].1, win[3].1] {
                        first_occurence = win[3].0;
                        break;
                    }
                }
                temporary_total += first_occurence;
            }

            Some(temporary_total)
        })
        .max()
        .unwrap_or(0);

    format!("{}", most_bananas)
}

fn get_next_secret(secret: isize) -> isize
{
    let step1 = ((secret << 6) ^ secret) % 16777216;
    let step2 = ((step1 >> 5) ^ step1) % 16777216;
    let step3 = ((step2 << 11) ^ step2) % 16777216;
    step3
}
