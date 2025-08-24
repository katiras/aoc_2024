use std::io::{self, Read};

fn main() {
    let file = std::fs::File::open("./input.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut input = String::new();

    reader.read_to_string(&mut input).unwrap();

    let mut total = 0;

    let mul_splits = input.split("mul(").collect::<Vec<&str>>();

    let mut last_is_do = true;

    for i in 1..mul_splits.len() {
        let do_i = find_substring_indices(mul_splits[i - 1], "do()");
        let dont_i = find_substring_indices(mul_splits[i - 1], "don't()");

        if do_i.len() > 0 && dont_i.len() > 0 {
            if do_i.iter().max() > dont_i.iter().max() {
                last_is_do = true;
            }

            if dont_i.iter().max() > do_i.iter().max() {
                last_is_do = false;
            }
        }
        
        if do_i.len() > 0 {
            last_is_do = true;
        }
        
        if dont_i.len() > 0 {
            last_is_do = false;
        }

        let sub_string = mul_splits[i];

        if !last_is_do {
            continue;
        }

        if sub_string.chars().any(|x| x == ')') {
            let mul_args_str = sub_string.split(")").next().unwrap();

            if mul_args_str.chars().filter(|x| *x == ',').count() == 1 {
                let mut mul_args = mul_args_str.split(",");

                match (mul_args.next(), mul_args.next()) {
                    (Some(first), Some(second)) => {
                        if first.parse::<usize>().is_ok() && second.parse::<usize>().is_ok() {
                            total +=
                                first.parse::<usize>().unwrap() * second.parse::<usize>().unwrap()
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    println!("{:?}", total)
}

fn find_substring_indices(s: &str, substr: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut pos = 0;

    while let Some(idx) = s[pos..].find(substr) {
        indices.push(pos + idx);
        pos += idx + substr.len();
    }

    indices
}
