use std::{
    cmp::Ordering,
    collections::HashSet,
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

    let mut unsorted_update_indexes: HashSet<usize> = HashSet::new();

    for rule in &rules {
        for (i, update) in updates.iter_mut().enumerate() {
            if let (Some(before_index), Some(after_index)) = (
                update.iter().position(|&x| x == rule.before),
                update.iter().position(|&x| x == rule.after),
            ) {
                if before_index > after_index {
                    unsorted_update_indexes.insert(i);
                }
            }
        }
    }

    let mut unsorted_updates = <Vec<Vec<usize>>>::new();

    for (i, update) in updates.iter().enumerate() {
        if !unsorted_update_indexes.contains(&i) {
            sum += update[update.len() / 2];
        } else {
            unsorted_updates.push(update.to_vec());
        }
    }

    println!("{:?}", sum);

    let mut sorted_sum = 0;

    for mut update in unsorted_updates {
        update.sort_by(|a, b| compare(&rules, *a, *b));
        sorted_sum += update[update.len() / 2]
    }

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
