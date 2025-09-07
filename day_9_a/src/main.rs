use std::fs;
const RADIX: u32 = 10;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let chars: Vec<char> = contents.chars().collect();

    let mut blocks: Vec<String> = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;

    for c in chars {
        let digit = c.to_digit(RADIX).unwrap();

        if is_file {
            for _ in 0..digit {
                blocks.push(file_id.to_string());
            }
            file_id += 1;
        } else {
            for _ in 0..digit {
                blocks.push(".".to_string());
            }
        }
        is_file = !is_file
    }

    let mut i = 0;
    let mut j = blocks.len() - 1;

    while i < j {
        if blocks[i] == "." && blocks[j] != "." {
            blocks.swap(i, j);
            i += 1;
            j -= 1;
        } else {
            if blocks[i] != "." {
                i += 1;
            }
            if blocks[j] == "." {
                j -= 1;
            }
        }
    }
    
    let mut sum: u64 = 0;
    
    for (i, str) in blocks.iter().enumerate() {
        if str != "." {
            sum += str.parse::<u64>().unwrap() * i as u64
        }
    }

    println!("{:?}", sum);
}
