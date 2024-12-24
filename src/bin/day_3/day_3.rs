use std::fs::File;
use std::io::{self, BufReader, Read};
use std::iter::Peekable;
use std::num::ParseIntError;
use std::result;

use aoc2024::get_reader;

#[derive(Debug, PartialEq)]
enum Token {
    WhiteSpace,
    OpenParen,
    ClosingParen,
    Comma,
    Punctuation(char),
    String(String),
    Number(usize),
}

fn main() -> io::Result<()> {
    let reader = get_reader("resources/day_3.txt")?;

    let tokens = lex(reader);
    let res = sum_all_multiplications(&tokens);

    println!("sum multiplications: {res}");

    Ok(())
}

fn filter_instructions(slice: &[Token]) -> bool {
    if let [string, open_paren, a, comma, b, closing_paren] = slice {
        *string == Token::String(String::from("mul"))
            && *open_paren == Token::OpenParen
            && std::mem::discriminant(a) == std::mem::discriminant(&Token::Number(0))
            && *comma == Token::Comma
            && std::mem::discriminant(b) == std::mem::discriminant(&Token::Number(0))
            && *closing_paren == Token::ClosingParen
    } else {
        false
    }
}

fn sum_all_multiplications(tokens: &[Token]) -> usize {
    tokens
        .windows(6)
        .filter(|&window| filter_instructions(window))
        .map(|window| {
            window
                .iter()
                .filter(|&e| std::mem::discriminant(e) == std::mem::discriminant(&Token::Number(0)))
                .collect::<Vec<_>>()
        })
        .fold(0, |acc, v| {
            let result = if let [a, b] = v.as_slice() {
                match (a, b) {
                    (Token::Number(_a), Token::Number(_b)) => *_a * *_b,
                    _ => 0,
                }
            } else {
                0
            };

            acc + result
        })
}

fn lex(reader: BufReader<File>) -> Vec<Token> {
    let mut res = Vec::new();

    let mut stream = reader.bytes().peekable();
    while let Some(byte) = stream.peek() {
        let ascii = byte.as_ref().unwrap().clone();
        let token = if ascii.is_ascii_whitespace() {
            stream.next();
            Token::WhiteSpace
        } else if ascii == b'(' {
            stream.next();
            Token::OpenParen
        } else if ascii == b')' {
            stream.next();
            Token::ClosingParen
        } else if ascii == b',' {
            stream.next();
            Token::Comma
        } else if ascii.is_ascii_punctuation() {
            stream.next();
            Token::Punctuation(ascii as char)
        } else if ascii.is_ascii_digit() {
            Token::Number(number(&mut stream).unwrap())
        } else {
            Token::String(string(&mut stream))
        };
        res.push(token);
    }

    res
}

fn number<I>(stream: &mut Peekable<I>) -> result::Result<usize, ParseIntError>
where
    I: Iterator<Item = io::Result<u8>>,
{
    let mut buf = String::new();
    while let Some(byte) = stream.peek() {
        let ascii = byte.as_ref().unwrap().clone();
        if !ascii.is_ascii_digit() {
            break;
        }

        buf.push(ascii as char);
        stream.next();
    }
    buf.parse::<usize>()
}

fn string<I>(stream: &mut Peekable<I>) -> String
where
    I: Iterator<Item = io::Result<u8>>,
{
    let mut buf = String::new();
    while let Some(byte) = stream.peek() {
        let ascii = byte.as_ref().unwrap().clone();
        if !ascii.is_ascii_alphabetic() {
            break;
        }

        buf.push(ascii as char);
        stream.next();
    }
    buf
}
