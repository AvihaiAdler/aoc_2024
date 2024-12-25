use aoc2024::get_reader;
use std::fs;
use std::io::{self, BufRead, BufReader};

mod direction;
use direction::{Direction, Orientation};

fn main() -> io::Result<()> {
    let reader = get_reader("resources/day_4.txt")?;
    let data = get_data(reader);

    let needle = "XMAS";
    let words = search_for(&data, needle);
    println!("found {} occurrences of '{needle}'", words.iter().count());

    let words = search_for(&data, &needle[1..]);

    /*
     * get all coord with diagonal orientation
     * adjust those coord such that they point to the letter 'A' (pivot)
     * get all permutations of those coords
     * find pairs such that they share the same pivot but their orientation is different
     */
    let diagonals = words
        .iter()
        .filter(|&dir| Orientation::diagonal(dir.orientation))
        .map(|dir| adjust_coord(dir))
        .collect::<Vec<_>>();

    let crossed = diagonals
        .iter()
        .map(|first| {
            diagonals
                .iter()
                .map(|second| (first.clone(), second.clone()))
        })
        .flatten()
        .filter(|(first, second)| first.orientation != second.orientation)
        .filter(|(first, second)| first.orientation.inverse() != second.orientation)
        .filter(|(first, second)| first.row == second.row)
        .filter(|(first, second)| first.col == second.col)
        .count();

    println!("found {} diagonally crossed 'MAS'", crossed / 2);
    Ok(())
}

/**
 * adjust the coordination such that the point represent the letter 'A'
 */
fn adjust_coord(direction: &Direction) -> Direction {
    match direction.orientation {
        Orientation::DiagDownLeft => {
            Direction::new(direction.orientation, direction.row + 1, direction.col - 1)
        }
        Orientation::DiagDownRight => {
            Direction::new(direction.orientation, direction.row + 1, direction.col + 1)
        }
        Orientation::DiagUpLeft => {
            Direction::new(direction.orientation, direction.row - 1, direction.col - 1)
        }
        Orientation::DiagUpRight => {
            Direction::new(direction.orientation, direction.row - 1, direction.col + 1)
        }
        _ => direction.clone(),
    }
}

fn search(data: &[Vec<char>], direction: &Direction, string: &str) -> Option<Orientation> {
    let c = string.chars().next();
    if c == None {
        return Some(direction.orientation);
    }

    if data.get(direction.row) == None {
        return None;
    }

    if data[direction.row].get(direction.col) == None {
        return None;
    }

    if data[direction.row][direction.col] != c.unwrap() {
        return None;
    }

    return search(data, &direction.advance(), &string[1..]);
}

/**
 * search for a string.
 * returns Direction where Direction::row & Direction::col point to the last letter in the string,
 * and Direction::Orientation represent the orientation of the string as if it was read backwards
 */
fn search_for(data: &[Vec<char>], string: &str) -> Vec<Direction> {
    let string = string.chars().rev().collect::<String>();

    let rows = data.len();
    let cols = data[0].len();

    let mut res = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if data[row][col] != string.chars().next().unwrap() {
                continue;
            }

            let needles = [
                Orientation::Up,
                Orientation::Down,
                Orientation::Left,
                Orientation::Right,
                Orientation::DiagDownLeft,
                Orientation::DiagDownRight,
                Orientation::DiagUpLeft,
                Orientation::DiagUpRight,
            ]
            .iter()
            .map(|orientation| Direction::new(*orientation, row, col))
            .map(|direction| direction.advance())
            .map(|direction| search(data, &direction, &string[1..]))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|orientation| Direction::new(orientation, row, col))
            .collect::<Vec<_>>();

            res = res.iter().chain(needles.iter()).cloned().collect();
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
