use std::io::{self, BufRead};

fn main() {
    let file = std::fs::File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut count: usize = 0;

    for line in reader.lines() {
        let report = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_any_safe(report) {
            count += 1
        }
    }

    println!("{:?}\n", count);
}

fn is_any_safe(report: Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut rep = report.clone();
        rep.remove(i);

        if is_safe(&rep) {
            return true;
        }

        if is_safe(&rep.into_iter().rev().collect::<Vec<i32>>()) {
            return true;
        }
    }
    return false;
}

fn is_safe(rep: &Vec<i32>) -> bool {
    if rep.is_sorted() {
        let diffs = rep.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();

        if diffs.into_iter().all(|x| x > 0 && x < 4) {
            return true;
        }
    }
    return false;
}
