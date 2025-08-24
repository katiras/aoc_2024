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

        if is_match(&list, None, total) {
            sum += total;
        }
    }

    println!("{:?}", sum);
}

fn is_match(remaining_items: &[usize], current_total: Option<usize>, total: usize) -> bool {
    if remaining_items.is_empty() || current_total.unwrap_or(0) > total {
        return current_total.unwrap() == total;
    }

    let (first, rest) = remaining_items.split_first().unwrap();

    return is_match(rest, Some(current_total.unwrap_or(0) + first), total)
        || is_match(rest, Some(current_total.unwrap_or(1) * first), total);
}
