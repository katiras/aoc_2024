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

fn matches_total(
    remaining_items: &[usize],
    current_total: Option<usize>,
    total: usize,
) -> bool {
    if remaining_items.is_empty() || current_total.unwrap_or(0) > total {
        return current_total.unwrap() == total;
    }

    let (first, rest) = remaining_items.split_first().unwrap();

    let mul_total = current_total.unwrap_or(1) * first;
    let add_total = current_total.unwrap_or(0) + first;
    let concat_total = concat(current_total.unwrap_or(0), *first);

    return matches_total(rest, Some(add_total), total)
        || matches_total(rest, Some(mul_total), total)
        || matches_total(rest, Some(concat_total), total);
}

fn concat(a: usize, b: usize) -> usize {
    let multiplier = 10_usize.pow(count_digits(b));

    return a * multiplier + b;
}

fn count_digits(n: usize) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;
    let mut num = n;

    while num > 0 {
        num /= 10;
        count += 1;
    }

    return count;
}
