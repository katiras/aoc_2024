use std::io::{self, BufRead};

struct Direction {
    row_offset: i32,
    col_offset: i32,
}

fn main() {
    let file = std::fs::File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    let top_right = &Direction {
        row_offset: 1,
        col_offset: -1,
    };

    let top_left = &Direction {
        row_offset: -1,
        col_offset: -1,
    };

    let bottom_right = &Direction {
        row_offset: 1,
        col_offset: 1,
    };

    let bottom_left = &Direction {
        row_offset: -1,
        col_offset: 1,
    };

    let mut xmas_count = 0;

    for line in reader.lines().map_while(Result::ok) {
        grid.push(line.chars().collect());
    }

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if is_word(&grid, x, y, top_right, "AS")
                && is_word(&grid, x, y, bottom_right, "AS")
                && is_word(&grid, x, y, top_left, "AM")
                && is_word(&grid, x, y, bottom_left, "AM")
            {
                xmas_count += 1
            }
            if is_word(&grid, x, y, top_right, "AS")
                && is_word(&grid, x, y, bottom_right, "AM")
                && is_word(&grid, x, y, top_left, "AS")
                && is_word(&grid, x, y, bottom_left, "AM")
            {
                xmas_count += 1
            }
            if is_word(&grid, x, y, top_right, "AM")
                && is_word(&grid, x, y, bottom_right, "AM")
                && is_word(&grid, x, y, top_left, "AS")
                && is_word(&grid, x, y, bottom_left, "AS")
            {
                xmas_count += 1
            }
            if is_word(&grid, x, y, top_right, "AM")
                && is_word(&grid, x, y, bottom_right, "AS")
                && is_word(&grid, x, y, top_left, "AM")
                && is_word(&grid, x, y, bottom_left, "AS")
            {
                xmas_count += 1
            }
        }
    }

    println!("{:?}", xmas_count)
}

fn is_word(grid: &Vec<Vec<char>>, x: usize, y: usize, d: &Direction, word: &str) -> bool {
    match get_word(grid, x as i32, y as i32, &d, word.len()) {
        Some(found_word) => return found_word == word,
        None => return false,
    }
}

fn get(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Option<&char> {
    grid.get(y).and_then(|row| row.get(x))
}

fn get_word(
    grid: &Vec<Vec<char>>,
    row: i32,
    col: i32,
    direction: &Direction,
    word_length: usize,
) -> Option<String> {
    let mut word_found = String::new();
    let mut current_distance = 0;
    let mut x = row + direction.row_offset * current_distance;
    let mut y = col + direction.col_offset * current_distance;

    while x > -1
        && x < grid[0].len() as i32
        && y > -1
        && y < grid.len() as i32
        && current_distance < word_length as i32
    {
        let next_char = get(grid, x as usize, y as usize).unwrap();

        word_found.push(*next_char);

        current_distance += 1;
        x = row + direction.row_offset * current_distance;
        y = col + direction.col_offset * current_distance;
    }

    if word_found.len() == word_length {
        return Some(word_found);
    }

    return None;
}
