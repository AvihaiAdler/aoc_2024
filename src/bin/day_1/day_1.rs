use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::zip;

use aoc2024::get_reader;

fn main() -> io::Result<()> {
    let reader = get_reader("resources/day_1.txt")?;

    let all_numbers = get_all_numbers(reader);
    let (mut first, mut second) = split(&all_numbers);

    println!("distance: {}", sum_distance(&mut first, &mut second));

    let occurrencess = find_occurrences(&first, &second);
    println!("similarity score: {}", similarity_score(occurrencess));

    Ok(())
}

fn find_occurrences(first: &[i32], second: &[i32]) -> HashMap<i32, usize> {
    let mut occurrences = HashMap::<i32, usize>::new();
    first.iter().for_each(|elem| {
        occurrences
            .entry(*elem)
            .or_insert_with(|| second.iter().filter(|&e| *e == *elem).count());
    });

    occurrences
}

fn similarity_score(occurrencess: HashMap<i32, usize>) -> usize {
    occurrencess
        .iter()
        .fold(0, |acc, (key, value)| acc + *key as usize * value)
}

fn get_all_numbers(reader: BufReader<File>) -> Vec<i32> {
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn split(numbers: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let (first, second): (Vec<_>, Vec<_>) =
        numbers.iter().enumerate().partition(|tup| tup.0 % 2 == 0);

    (
        first.iter().map(|(_, &elem)| elem).collect(),
        second.iter().map(|(_, &elem)| elem).collect(),
    )
}

fn sum_distance(first: &mut [i32], second: &mut [i32]) -> i32 {
    first.sort();
    second.sort();

    zip(first, second)
        .map(|(a, b)| (*a - *b).abs())
        .fold(0, |acc, i| acc + i)
}
