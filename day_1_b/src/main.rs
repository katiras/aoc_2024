use std::io::{self, BufRead};

fn main() {
    let file = std::fs::File::open("./input.txt");
    let lines = io::BufReader::new(file.unwrap()).lines();

    let mut first_column: Vec<usize> = Vec::new();
    let mut second_column: Vec<usize> = Vec::new();

    for line in lines.map_while(Result::ok) {
        let first = &line[..5].parse::<usize>().unwrap();
        first_column.push(*first);

        let second = &line[8..13].parse::<usize>().unwrap();
        second_column.push(*second);
    }

    first_column.sort();
    second_column.sort();

    let mut total_occurances = 0;

    for n in first_column {
        let occurances = second_column.iter().filter(|&x| *x == n).count();
        total_occurances += occurances * n;
    }

    println!("{}", total_occurances);
}
