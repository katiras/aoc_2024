use std::io::{self, BufRead};

fn main() {
    let file = std::fs::File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut count: usize = 0;

    for line in reader.lines() {
        let reports = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if reports.is_sorted() {
            let diffs = reports
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i32>>();

            if diffs.iter().all(|x| *x > 0 && *x < 4) {
                // println!("{:?}", reports);
                // println!("{:?}\n", diffs);
                count += 1;
            }
        }

        let rev_reports = reports.into_iter().rev().collect::<Vec<i32>>();

        if rev_reports.is_sorted() {
            let diffs = rev_reports
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i32>>();

            if diffs.iter().all(|x| *x > 0 && *x < 4) {
                // println!("{:?}", rev_reports);
                // println!("{:?}\n", diffs);
                count += 1;
            }
        }
    }

    println!("{:?}\n", count);
}
