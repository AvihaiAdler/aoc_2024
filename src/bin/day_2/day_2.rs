use std::fs::File;
use std::io::{self, BufRead, BufReader};

use aoc2024::get_reader;

fn main() -> io::Result<()> {
    let reader = get_reader("resources/day_2.txt")?;

    let data = get_data(reader);
    println!("safe reports: {}", check_safety(&data));
    println!("relaxed reports: {}", check_safety_relaxed(&data));
    Ok(())
}

fn get_data(reader: BufReader<File>) -> Vec<Vec<i8>> {
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split_whitespace()
                .map(|itr| itr.parse::<i8>().unwrap())
                .collect()
        })
        .collect()
}

fn get_diff(data: &[i8]) -> Vec<i8> {
    data.windows(2)
        .map(|window| window[0] - window[1])
        .collect::<Vec<_>>()
}

fn check_diff_safety(diffs: &[i8]) -> bool {
    let signedness = diffs.windows(2).all(|window| window[0] * window[1] > 0);
    let limit_exceeded = diffs.iter().all(|e| e.abs() < 4);

    signedness & limit_exceeded
}

/**
 * checks for 'safe reports'. a 'safe report' is defined to be a report where all the differences between 2 adjacent elements have the same singedness,
 * and no difference is larger than 3    
 */
fn check_safety(data: &[Vec<i8>]) -> usize {
    data.iter()
        .map(|vec| get_diff(vec))
        .map(|diff| check_diff_safety(&diff))
        .fold(0, |acc, res| if res { acc + 1 } else { acc })
}

/**
 * brute force the thing. not very elegant
 */
fn check_safety_relaxed(data: &[Vec<i8>]) -> usize {
    let mut res = 0;
    for vec in data.iter() {
        for idx in 0..vec.len() {
            let mut tmp = vec.clone();
            tmp.remove(idx);
            let relaxed_diff = get_diff(&tmp);

            if check_diff_safety(&relaxed_diff) {
                res += 1;
                break;
            }
        }
    }
    res
}
