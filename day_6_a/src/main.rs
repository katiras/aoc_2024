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

    let mut guard = Guard::new(&grid);

    let mut visited_tiles: HashSet<(usize, usize)> = HashSet::new();

    while guard.is_inside_grid(&grid) {

        let current_char = grid[guard.r as usize][guard.c as usize];

        if current_char == '#' {
            guard.go_backwards();
            guard.rotate_clockwise();
        }

        visited_tiles.insert((guard.r as usize, guard.c as usize));
        guard.go_forward();
    }

    println!("{:?}", visited_tiles.len());
}

#[derive(Debug)]
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
        match (self.c_direction, self.r_direction) {
            (0, -1) => {
                self.c_direction = 1;
                self.r_direction = 0;
            }
            (1, 0) => {
                self.c_direction = 0;
                self.r_direction = 1;
            }
            (0, 1) => {
                self.c_direction = -1;
                self.r_direction = 0;
            }
            (-1, 0) => {
                self.c_direction = 0;
                self.r_direction = -1;
            }
            _ => panic!("Invalid guard direction"),
        }
    }

    fn go_forward(&mut self) {
        self.c = self.c + self.c_direction;
        self.r = self.r + self.r_direction;
    }

    fn go_backwards(&mut self) {
        self.c = self.c - self.c_direction;
        self.r = self.r - self.r_direction;
    }

    fn is_inside_grid(&self, grid: &Vec<Vec<char>>) -> bool {
        self.r < grid.len() as i32 && self.r > -1 && self.c < grid[0].len() as i32 && self.c > -1
    }
}
