use std::io::{self, BufRead};

#[derive(Debug)]
struct Rule {
    before: usize,
    after: usize,
}

fn main() {
    let file = std::fs::File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut updates: Vec<Vec<usize>> = Vec::new();
    let mut rules: Vec<Rule> = Vec::new();

    let mut sum = 0;

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

    let mut invalid_update_indexes: Vec<usize> = Vec::new();

    for rule in rules {
        for (i, update) in updates.iter().enumerate() {
            if let (Some(before_index), Some(after_index)) = (
                update.iter().position(|&x| x == rule.before),
                update.iter().position(|&x| x == rule.after),
            ) {
                if before_index > after_index {
                    invalid_update_indexes.push(i);
                    continue;
                }
            }
        }
    }

    invalid_update_indexes.sort_unstable();
    invalid_update_indexes.dedup();

    for (i, update) in updates.iter().enumerate() {
        if !invalid_update_indexes.contains(&i) {
            sum += update[update.len() / 2];
        }
    }

    println!("{:?}", sum);
}
