use std::env;
use std::fs::File;
use std::io::{BufRead, Read};
use std::ops::Add;

#[inline]
fn get_value1(line: &str) -> u32 {
    let mut first = '\0';
    let mut last = '\0';

    for c in line.chars() {
        if c.is_ascii_digit() {
            if first == '\0' {
                first = c;
            }
            last = c;
        }
    }

    10 * first as u32 + last as u32 - 0x210
}

#[inline]
fn get_value2(line: &str) -> u32 {
    let mut first = u32::MAX;
    let mut last = 0;
    const DIGITS: [(&str, u32); 9] =
        [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for x in 0..line.len() {
        let slice = &line[x..];
        let next_char = slice.chars().next().unwrap();
        let value = next_char.to_digit(10).or(DIGITS.iter().filter_map(|&(str, digit)| if slice.starts_with(str) { Some(digit) } else { None }).next());

        if value.is_some() {
            if first == u32::MAX {
                first = value.unwrap();
            }
            last = value.unwrap();
        }
    }

    10 * first + last
}

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    for line in s.lines() {
        part1 += get_value1(line);
        part2 += get_value2(line);
    }

    println!("{}\n{}", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(get_value1("1abc2"), 12);
    }
}
