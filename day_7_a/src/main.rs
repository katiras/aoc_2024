use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines().map_while(Result::ok) {
        let mut split = line.split(": ");

        let total = split.next().unwrap().parse::<usize>().unwrap();
        let list = split
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if matches_total(&list, None, total) {
            sum += total;
        }
    }

    println!("{:?}", sum);
}

fn matches_total(remaining_items: &[usize], current_total: Option<usize>, match_number: usize) -> bool {
    if remaining_items.is_empty() {
        return current_total.unwrap() == match_number;
    }

    let (first, rest) = remaining_items.split_first().unwrap();

    return matches_total(rest, Some(current_total.unwrap_or(0) + first), match_number)
        || matches_total(rest, Some(current_total.unwrap_or(1) * first), match_number);
}
