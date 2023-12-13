extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, iter};

#[inline(always)]
pub fn calculate_wins((time, distance) : (isize, isize)) -> isize {
    let s = ((time*time-4*distance) as f64).sqrt() / 2f64;
    let is_odd = time & 1;
    ((0.5f64 * is_odd as f64 + s).ceil() as isize) * 2 - is_odd - 1
}

#[inline(always)]
pub fn get_value(line: &str) -> isize {
    line.chars()
        .skip_while(|c| *c != ':' )
        .skip(1)
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse::<isize>().unwrap()
}

fn main() {
    let mut file = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap());

    let mut lines = file.lines();
    let mut time_line = lines.next().unwrap().unwrap();
    let mut distance_line = lines.next().unwrap().unwrap();
    let result = iter::zip(
        time_line[12..].split_ascii_whitespace().map(|t| t.parse::<isize>().unwrap()),
        distance_line[11..].split_ascii_whitespace().map(|t| t.parse::<isize>().unwrap())
    ).map(calculate_wins)
        .product::<isize>();

    let time = get_value(&time_line);
    let distance = get_value(&distance_line);
    println!("{}\n{}", result, calculate_wins((time, distance)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(calculate_wins((7, 9)), 4);
        assert_eq!(calculate_wins((15, 40)), 8);
        assert_eq!(calculate_wins((30, 200)), 9);
    }

    #[test]
    fn test_input() {
        assert_eq!(calculate_wins((53, 333)), 38);
        assert_eq!(calculate_wins((83, 1635)), 18);
        assert_eq!(calculate_wins((72, 1289)), 5);
        assert_eq!(calculate_wins((88, 1532)), 41);
    }
}