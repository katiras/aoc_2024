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

impl Point {
    fn is_inside_grid(self: &Point, max_x: i32, max_y: i32) -> bool {
        return self.x > -1 && self.x <= max_x && self.y > -1 && self.y <= max_y;
    }
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

                for multiplier in 0.. {
                    let antinode_a = Point {
                        x: popped_antenna.x - diff.x * multiplier,
                        y: popped_antenna.y - diff.y * multiplier,
                    };

                    if !&antinode_a.is_inside_grid(max_x, max_y) {
                        break;
                    }

                    antinodes.insert(antinode_a);
                }

                for multiplier in 0.. {
                    let antinode_b = Point {
                        x: a.x + diff.x * multiplier,
                        y: a.y + diff.y * multiplier,
                    };

                    if !&antinode_b.is_inside_grid(max_x, max_y) {
                        break;
                    }

                    antinodes.insert(antinode_b);
                }
            }
        }
    }

    let sum = antinodes.iter().count();

    println!("{:?}", sum);
}
