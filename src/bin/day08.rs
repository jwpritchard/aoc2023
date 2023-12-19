use std::fs::File;
use std::io::Read;
use nohash_hasher::IntMap;
use num_integer::Integer;
use aoc2023::common::file::MmapFile;

fn path_length(directions: &str, start_node: u64, end: impl Fn(u64) -> bool, nodes: &IntMap<u64, (u64, u64)>) -> u64 {
    let mut dir = directions.chars().cycle();
    let mut current_node = start_node;
    let mut length = 0;

    while !end(current_node) {
        let (left, right) = nodes[&current_node];
        current_node = if dir.next().unwrap() == 'L' { left } else { right };
        length = length + 1;
    }
    length
}

fn get_node_int(s: &str) -> u64 {
    let mut bytes = s.bytes();
    (bytes.next().unwrap() as u64) << 16
        | (bytes.next().unwrap() as u64) << 8
        | (bytes.next().unwrap() as u64)
}

fn parse_line(line: &str) -> (u64, (u64, u64)) {
    (get_node_int(&line[0..3]), (get_node_int(&line[7..10]), get_node_int(&line[12..15])))
}

fn main() {
    let mut file = MmapFile::from_args();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let directions = s.lines().next().unwrap();
    let nodes = s.lines().skip(2).map(parse_line)
        .collect::<IntMap<u64, (u64, u64)>>();

    let part1 = path_length(directions, 0x414141, |n| n == 0x5A5A5A, &nodes);

    let part2 = nodes.keys().filter(|&k| (k & 0xFF) == 0x41)
        .map(|&n| path_length(directions, n, |n| n & 0xFF == 0x5A, &nodes))
        .reduce(|i, j| i.lcm(&j))
        .unwrap();

    println!("{}\n{}", part1, part2);
}
