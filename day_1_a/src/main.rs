use std::io::{self, BufRead};

fn main() {
    let file = std::fs::File::open("./input.txt");
    let lines = io::BufReader::new(file.unwrap()).lines();

    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    for line in lines.map_while(Result::ok) {
        let first = &line[..5].parse::<i32>().unwrap();
        first_column.push(*first);

        let second = &line[8..13].parse::<i32>().unwrap();
        second_column.push(*second);
    }

    first_column.sort();
    second_column.sort();

    let mut total_diff = 0;

    for n in 0..first_column.len() {
        let diff = first_column[n] - second_column[n];
        total_diff += diff.abs();
    }

    println!("{}", total_diff);
}
