use std::fs::File;
use std::io::{self, BufRead, BufReader};

use aoc2024::get_reader;

fn main() -> io::Result<()> {
    let reader = get_reader("resources/day_2.txt")?;

    let data = get_data(reader);
    println!("safe reports: {}", check_diffs_safety(&data));
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

fn get_diffs(data: &[Vec<i8>]) -> Vec<Vec<i8>> {
    data.iter()
        .map(|v| {
            v.windows(2)
                .map(|window| window[0] - window[1])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn check_diffs_safety(data: &[Vec<i8>]) -> usize {
    let diffs = get_diffs(data);

    // filter out all the diffs where 2 adjecent diffs have different signedness
    diffs
        .iter()
        .filter(|&diff| diff.windows(2).all(|window| window[0] * window[1] > 0))
        .filter(|&diff| diff.iter().all(|&e| e.abs() < 4))
        .count()
}

fn check_diff_safety(diffs: &[i8]) -> bool {
    let signedness = diffs.windows(2).all(|window| window[0] * window[1] > 0);
    let limit_exceeded = diffs.iter().all(|e| e.abs() < 4);

    signedness & limit_exceeded
}

fn get_diff(data: &[i8]) -> Vec<i8> {
    data.windows(2)
        .map(|window| window[0] - window[1])
        .collect::<Vec<_>>()
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

#[allow(unused)]
fn check_safety_lossly(data: &[Vec<i8>]) -> usize {
    let diffs = get_diffs(data);

    // TODO
    let fluctuating_diffs = diffs
        .iter()
        .filter(|&diff| diff.windows(2).any(|window| window[0] * window[1] <= 0))
        .filter(|&diff| {
            // filter all fluctuating diffs with more than 1 fluctuation
            diff.windows(2)
                .map(|window| window[0] * window[1])
                .filter(|&e| e < 0)
                .count()
                > 1
        })
        .cloned()
        .collect::<Vec<_>>();

    let sum_of_fluctuating_diffs = fluctuating_diffs
        .iter()
        .filter(|&diff| diff.iter().all(|&e| e.abs() <= 3))
        .map(|diff| {
            diff.windows(2)
                .map(|window| window[0] + window[1])
                .collect::<Vec<_>>()
        })
        .count();

    // get all sum of all unsafe diffs. filter out all 'fluctuating' diffs
    let sum_of_diffs = diffs
        .iter()
        .filter(|&diff| diff.windows(2).all(|window| window[0] * window[1] > 0))
        .filter(|&diff| diff.iter().any(|&e| e.abs() > 3))
        .map(|diff| {
            diff.windows(2)
                .map(|window| window[0] + window[1])
                .collect::<Vec<_>>()
        })
        .filter(|diff| diff.iter().filter(|&e| e.abs() > 3).count() > 1) // filter out all of the sums of diffs who contain more than one 'usafe value'. i.e. [1, 2, 4, 4] will be filtered out
        .collect::<Vec<_>>();

    sum_of_diffs.len() + sum_of_fluctuating_diffs
}
