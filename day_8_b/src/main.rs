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

                let mut multiplier = 0;

                loop {
                    let antinode_a = Point {
                        x: popped_antenna.x - diff.x * multiplier,
                        y: popped_antenna.y - diff.y * multiplier,
                    };

                    if !is_inside_grid(&antinode_a, max_x, max_y) {
                        break;
                    }

                    antinodes.insert(antinode_a);
                    multiplier += 1;
                }

                multiplier = 0;

                loop {
                    let antinode_b = Point {
                        x: a.x + diff.x * multiplier,
                        y: a.y + diff.y * multiplier,
                    };

                    if !is_inside_grid(&antinode_b, max_x, max_y) {
                        break;
                    }

                    antinodes.insert(antinode_b);
                    multiplier += 1;
                }
            }
        }
    }

    let sum = antinodes.iter().count();

    println!("{:?}", sum);
}

fn is_inside_grid(point: &Point, max_x: i32, max_y: i32) -> bool {
    return point.x > -1 && point.x <= max_x && point.y > -1 && point.y <= max_y;
}
