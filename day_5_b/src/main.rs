use std::{
    cmp::Ordering,
    io::{self, BufRead},
};

#[derive(Debug, Clone)]
struct Rule {
    before: usize,
    after: usize,
}

fn main() {
    let file = std::fs::File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut updates: Vec<Vec<usize>> = Vec::new();
    let mut rules: Vec<Rule> = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        if line.len() > 5 {
            let update = line
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            updates.push(update);
        } else if line.len() == 5 {
            let (before, after) = line.split_once("|").unwrap();

            rules.push(Rule {
                before: before.parse::<usize>().unwrap(),
                after: after.parse::<usize>().unwrap(),
            });
        }
    }

    let mut already_sorted_count = 0;
    let mut sorted_sum = 0;

    for mut update in updates {
        if update.is_sorted_by(|a, b| compare(&rules, *a, *b) == Ordering::Less) {
            already_sorted_count += update[update.len() / 2];
        } else {
            update.sort_by(|a, b| compare(&rules, *a, *b));
            sorted_sum += update[update.len() / 2]
        }
    }

    println!("{:?}", already_sorted_count);
    println!("{:?}", sorted_sum);
}

fn compare(rules: &Vec<Rule>, a: usize, b: usize) -> Ordering {
    match rules
        .iter()
        .find(|x| x.before == a && x.after == b)
        .is_some()
    {
        true => Ordering::Less,
        false => Ordering::Greater,
    }
}
