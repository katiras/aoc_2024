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

        if report.is_sorted() {
            let diffs = report
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i32>>();

            if diffs.into_iter().all(|x| x > 0 && x < 4) {
                // println!("{:?}", reports);
                // println!("{:?}\n", diffs);
                count += 1;
            }
        }

        let rev_report = report.into_iter().rev().collect::<Vec<i32>>();

        if rev_report.is_sorted() {
            let diffs = rev_report
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i32>>();

            if diffs.into_iter().all(|x| x > 0 && x < 4) {
                // println!("{:?}", rev_reports);
                // println!("{:?}\n", diffs);
                count += 1;
            }
        }
    }

    println!("{:?}\n", count);
}
