use std::fs::File;
use std::io::{self, BufReader, Read};

use aoc2024::get_reader;

fn main() -> io::Result<()> {
    let reader = get_reader("resources/day_3.txt")?;
    let res = scan_input(reader);

    let res = res
        .iter()
        .fold(0, |acc, tup| acc + (tup.0 as usize * tup.1 as usize));
    println!("{}", res);

    Ok(())
}

fn scan_input(reader: BufReader<File>) -> Vec<(i16, i16)> {
    let sequence = "mul(";

    let mut res = Vec::new();

    let mut idx = 0;
    let mut op = None;
    let mut number = String::new();
    for byte in reader.bytes() {
        let ascii = byte.unwrap();

        if idx < sequence.len() && ascii == sequence.chars().nth(idx).unwrap() as u8 {
            idx += 1;
            continue;
        } else if idx < sequence.len() || idx > sequence.len() {
            idx = 0;
            continue;
        }

        if ascii.is_ascii_digit() {
            number.push(ascii as char);
            continue;
        } else if ascii == b',' as u8 && op.is_none() {
            op = Some(number.parse().unwrap());
            number.clear();
            continue;
        } else if ascii == b')' && op.is_some() {
            res.push((op.unwrap(), number.parse().unwrap()));
        }

        number.clear();
        op = None;
        idx = 0;
    }

    res
}
