use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut antennas_per_frequency = HashMap::<char, Vec<Point>>::new();

    let mut antinodes = HashSet::<Point>::new();

    let mut max_y: i32 = 0;
    let mut max_x: i32 = 0;

    for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas_per_frequency
                    .entry(char)
                    .or_insert(Vec::new())
                    .push(Point {
                        x: x as i32,
                        y: y as i32,
                    })
            }

            max_x = x as i32;
        }

        max_y = y as i32;
    }

    for (_freq, mut antennas) in antennas_per_frequency {
        while !antennas.is_empty() {
            let popped_antenna = antennas.pop().unwrap();

            for a in &antennas {
                let diff = Point {
                    x: a.x - popped_antenna.x,
                    y: a.y - popped_antenna.y,
                };

                let antinode_a = Point {
                    x: popped_antenna.x - diff.x,
                    y: popped_antenna.y - diff.y,
                };
                let antinode_b = Point {
                    x: a.x + diff.x,
                    y: a.y + diff.y,
                };

                antinodes.insert(antinode_a);
                antinodes.insert(antinode_b);
            }
        }
    }

    let sum = antinodes
        .iter()
        .filter(|Point { y, x }| *y <= max_y && *y > -1 && *x <= max_x && *x > -1)
        .count();

    println!("{:?}", sum);
}
