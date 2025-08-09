use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        grid.push(line.chars().collect());
    }

    let mut sum = 0;

    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            if grid[row][column] == '.' {
                if is_guard_in_loop(grid.clone(), row as i32, column as i32) {
                    sum += 1;
                }
            }
        }
    }

    println!("{:?}", sum);
}

fn is_guard_in_loop(mut grid: Vec<Vec<char>>, obstacle_row: i32, obstacle_column: i32) -> bool {
    let mut guard = Guard::new(&grid);

    grid[obstacle_row as usize][obstacle_column as usize] = '#';

    let mut guard_states: HashSet<Guard> = HashSet::new();

    while guard.is_inside_grid(&grid) {
        if grid[guard.r as usize][guard.c as usize] == '#' {
            guard.go_backwards();
            guard.rotate_clockwise();
        }

        guard.go_forward();

        if !guard_states.insert(guard.clone()) {
            return true;
        }
    }

    return false;
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Guard {
    c: i32,
    r: i32,
    c_direction: i32,
    r_direction: i32,
}

impl Guard {
    fn new(grid: &Vec<Vec<char>>) -> Guard {
        for row in 0..grid.len() {
            for column in 0..grid[0].len() {
                if grid[row][column] == '^' {
                    return Guard {
                        c: column as i32,
                        r: row as i32,
                        c_direction: 0,
                        r_direction: -1,
                    };
                }
            }
        }

        panic!("Unable to find guard starting position");
    }

    fn rotate_clockwise(&mut self) {
        let new_c = -self.r_direction;
        let new_r = self.c_direction;

        self.c_direction = new_c;
        self.r_direction = new_r;
    }

    fn go_forward(&mut self) {
        self.c += self.c_direction;
        self.r += self.r_direction;
    }

    fn go_backwards(&mut self) {
        self.c -= self.c_direction;
        self.r -= self.r_direction;
    }

    fn is_inside_grid(&self, grid: &Vec<Vec<char>>) -> bool {
        self.r < grid.len() as i32 && self.r > -1 && self.c < grid[0].len() as i32 && self.c > -1
    }
}
