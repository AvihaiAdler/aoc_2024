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
    Do,
    Dont,
    Mul(usize, usize),
    Punctuation(char),
    String(String),
    Number(usize),
}

fn main() -> io::Result<()> {
    let reader = get_reader("resources/day_3.txt")?;

    let tokens = second_pass(&lex(reader));
    let res = sum_all_multiplications(&tokens);

    println!("sum multiplications: {res}");

    let res = sum_narrow_multiplications(&tokens);
    println!("sum narrow multiplications: {res}");

    Ok(())
}

fn sum_narrow_multiplications(tokens: &[Token]) -> usize {
    let mut res = 0;
    let mut flag = true;
    for token in tokens.iter() {
        match *token {
            Token::Do => flag = true,
            Token::Dont => flag = false,
            Token::Mul(a, b) => {
                if flag {
                    res += a * b
                }
            }
            _ => continue,
        }
    }

    res
}

fn sum_all_multiplications(tokens: &[Token]) -> usize {
    tokens
        .iter()
        .filter(|&token| std::mem::discriminant(token) == std::mem::discriminant(&Token::Mul(0, 0)))
        .map(|token| {
            if let Token::Mul(a, b) = token {
                (*a, *b)
            } else {
                (0usize, 0usize)
            }
        })
        .fold(0, |acc, (a, b)| acc + a * b)
}

// prefer fold over eq. we don't want to compare the undelying values. rather we want to compare their discriminant
fn starts_with(stream: &[Token], expr: &[Token]) -> bool {
    stream
        .iter()
        .take(expr.len())
        .zip(expr)
        .fold(true, |acc, (first, second)| {
            acc & (std::mem::discriminant(first) == std::mem::discriminant(second))
        })
}

fn second_pass(tokens: &[Token]) -> Vec<Token> {
    let mul = [
        Token::Mul(0, 0),
        Token::OpenParen,
        Token::Number(0),
        Token::Comma,
        Token::Number(0),
        Token::ClosingParen,
    ];

    let do_inst = [Token::Do, Token::OpenParen, Token::ClosingParen];
    let dont_inst = [Token::Dont, Token::OpenParen, Token::ClosingParen];

    let mut tmp = Vec::new();
    for window in tokens.windows(mul.len()) {
        if starts_with(window, mul.as_slice()) {
            let op = window
                .iter()
                .filter(|&e| std::mem::discriminant(e) == std::mem::discriminant(&Token::Number(0)))
                .map(|e| match *e {
                    Token::Number(a) => a,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            if let [a, b] = op.as_slice() {
                tmp.push(Token::Mul(*a, *b));
            }
        } else if starts_with(window, dont_inst.as_slice()) {
            tmp.push(Token::Dont);
        } else if starts_with(window, do_inst.as_slice()) {
            tmp.push(Token::Do);
        }
    }

    tmp
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
        if ascii != b'\'' && !ascii.is_ascii_alphabetic() {
            break;
        }

        buf.push(ascii as char);
        stream.next();
    }
    buf
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
        } else if ascii != b'\'' && ascii.is_ascii_punctuation() {
            stream.next();
            Token::Punctuation(ascii as char)
        } else if ascii.is_ascii_digit() {
            Token::Number(number(&mut stream).unwrap())
        } else {
            let string = string(&mut stream);
            match string {
                // if the string starts with "do"/"don't"/"mul" but have more alphabetical chars after it - then its a string not a token
                s if s.as_str().ends_with("do") => Token::Do,
                s if s.as_str().ends_with("don't") => Token::Dont,
                s if s.as_str().ends_with("mul") => Token::Mul(0, 0),
                _ => Token::String(string),
            }
        };

        res.push(token);
    }

    res
}
