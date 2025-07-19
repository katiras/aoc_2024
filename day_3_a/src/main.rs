use std::io::{self, Read};

fn main() {
    let file = std::fs::File::open("./input.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut input = String::new();

    reader.read_to_string(&mut input).unwrap();

    let mut total = 0;

    for sub in input.split("mul(") {
        if sub.chars().any(|x| x == ')') {
            let mul_args_str = sub.split(")").next().unwrap();

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
