#![feature(portable_simd)]

use std::io::Read;
use std::simd::{i32x32, mask32x32};
use aoc2023::common::file::MmapFile;

#[inline]
fn get_value(init: &[i32; 32], count: usize) -> (i32, i32) {
    const ZERO: i32x32 = i32x32::from_array([0; 32]);

    let mut diffs = i32x32::from_slice(init);
    let mut reverse = diffs;
    let mut forward = diffs;

    let mut x = 1;
    loop {
        let mask = mask32x32::from_bitmask(u64::MAX << (count - x));
        diffs = mask.select(ZERO, diffs.rotate_elements_left::<1>() - diffs);

        if diffs == ZERO {
            break;
        }

        reverse = if x & 1 == 0 { reverse + diffs } else { reverse - diffs };
        forward = forward.rotate_elements_left::<1>() + diffs;
        x = x + 1;
    }
    (reverse[0], forward[count - x])
}

fn main() {
    let file = MmapFile::from_args();

    let mut arr: [i32; 32] = [0; 32];
    let (mut prev, mut next): (i32, i32) = (0, 0);
    let mut i = 0;
    let mut number: i32 = 0;
    let mut negative = false;

    for b in file.bytes() {
        let byte = b.unwrap();

        if byte.is_ascii_digit() {
            number = 10 * number + (byte & 0x0F) as i32;
        }
        else if byte == b'-' {
            negative = true;
        }
        else {
            arr[i] = if negative { -number } else { number };
            number = 0;
            negative = false;
            i = i + 1;

            if byte == b'\n' {
                let (l, r) = get_value(&arr, i);
                i = 0;
                prev += l;
                next += r;
            }
        }
    }

    println!("{}\n{}", next, prev);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(get_value(&[ 0, 3, 6, 9, 12, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], 6), (-3, 18));
        assert_eq!(get_value(&[ 1, 3, 6, 10, 15, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], 6), (0, 28));
        assert_eq!(get_value(&[ 10, 13, 16, 21, 30, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], 6), (5, 68));
    }
}
