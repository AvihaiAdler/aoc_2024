use aoc2024::get_reader;
use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Copy)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
    DiagUpLeft,
    DiagUpRight,
    DiagDownLeft,
    DiagDownRight,
}

struct Direction {
    direction: Orientation,
    row: usize,
    col: usize,
}

impl Direction {
    fn new(direction: Orientation, row: usize, col: usize) -> Self {
        Self {
            direction,
            row,
            col,
        }
    }

    fn advance(&self) -> Self {
        match self.direction {
            Orientation::Up => {
                Self::new(self.direction, self.row.wrapping_add_signed(-1), self.col)
            }
            Orientation::Down => {
                Self::new(self.direction, self.row.wrapping_add_signed(1), self.col)
            }
            Orientation::Left => {
                Self::new(self.direction, self.row, self.col.wrapping_add_signed(-1))
            }
            Orientation::Right => {
                Self::new(self.direction, self.row, self.col.wrapping_add_signed(1))
            }
            Orientation::DiagDownLeft => Self::new(
                self.direction,
                self.row.wrapping_add_signed(1),
                self.col.wrapping_add_signed(-1),
            ),
            Orientation::DiagDownRight => Self::new(
                self.direction,
                self.row.wrapping_add_signed(1),
                self.col.wrapping_add_signed(1),
            ),
            Orientation::DiagUpLeft => Self::new(
                self.direction,
                self.row.wrapping_add_signed(-1),
                self.col.wrapping_add_signed(-1),
            ),
            Orientation::DiagUpRight => Self::new(
                self.direction,
                self.row.wrapping_add_signed(-1),
                self.col.wrapping_add_signed(1),
            ),
        }
    }
}

fn main() -> io::Result<()> {
    let reader = get_reader("resources/day_4.txt")?;
    let data = get_data(reader);

    let needle = "XMAS";
    let amount = search_for(&data, needle);
    println!("found {amount} occurrences of '{needle}'");

    Ok(())
}

fn search(data: &[Vec<char>], direction: &Direction, string: &str) -> bool {
    let c = string.chars().next();
    if c == None {
        return true;
    }

    if data.get(direction.row) == None {
        return false;
    }

    if data[direction.row].get(direction.col) == None {
        return false;
    }

    if data[direction.row][direction.col] != c.unwrap() {
        return false;
    }

    return search(data, &direction.advance(), &string[1..]);
}

fn search_for(data: &[Vec<char>], string: &str) -> usize {
    let rows = data.len();
    let cols = data[0].len();

    let mut res = 0;
    for row in 0..rows {
        for col in 0..cols {
            if data[row][col] != string.chars().next().unwrap() {
                continue;
            }

            let lookups = [
                Direction::new(Orientation::Up, row, col).advance(),
                Direction::new(Orientation::Down, row, col).advance(),
                Direction::new(Orientation::Left, row, col).advance(),
                Direction::new(Orientation::Right, row, col).advance(),
                Direction::new(Orientation::DiagDownLeft, row, col).advance(),
                Direction::new(Orientation::DiagDownRight, row, col).advance(),
                Direction::new(Orientation::DiagUpLeft, row, col).advance(),
                Direction::new(Orientation::DiagUpRight, row, col).advance(),
            ];

            res += lookups
                .iter()
                .map(|direction| search(data, direction, &string[1..]))
                .fold(0, |acc, result| acc + if result { 1 } else { 0 });
        }
    }

    res
}

fn get_data(reader: BufReader<fs::File>) -> Vec<Vec<char>> {
    reader
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect()
}
