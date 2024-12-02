pub fn part1(input: Vec<String>) -> String {
    let mut safe_reports = 0;

    for s in input.iter() {
        let mut report: Vec<i32> = vec![];
        for level in s.split_whitespace() {
            report.push(level.parse::<i32>().unwrap());
        }

        if is_safe(report) {
            safe_reports += 1;
        }
    }

    format!("{}", safe_reports)
}

pub fn part2(input: Vec<String>) -> String {
    let mut safe_reports = 0;

    for s in input.iter() {
        let mut report: Vec<i32> = vec![];
        for level in s.split_whitespace() {
            report.push(level.parse::<i32>().unwrap());
        }

        for i in 0..report.len() {
            let mut report_clone = report.clone();
            report_clone.remove(i);
            if is_safe(report_clone) {
                safe_reports += 1;
                break;
            }
        }
    }

    format!("{}", safe_reports)
}

fn is_safe(report: Vec<i32>) -> bool {
    let is_valid_increase = report.windows(2).all(|window| window[1] > window[0] && window[1] - window[0] <= 3);
    let is_valid_decrease = report.windows(2).all(|window| window[0] > window[1] && window[0] - window[1] <= 3);

    if is_valid_increase || is_valid_decrease {
        return true
    }

    false
}
