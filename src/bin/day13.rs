#![feature(portable_simd)]

use std::io::Read;
use std::simd::{mask32x32, u32x32};
use std::simd::cmp::SimdPartialEq;
use aoc2023::common::file::MmapFile;

fn get_match_position(values: &[u32], window_size: usize, exact: bool) -> isize {
    const ZERO: u32x32 = u32x32::from_array([0; 32]);
    let v = u32x32::from_slice(values);
    let mut w = v.reverse();
    let mask_clamp = u64::MAX >> (64 - window_size);
    let window = mask_clamp << window_size;

    for i in 1..window_size {
        w = w.rotate_elements_right::<2>();
        let mask = (window >> (2 * window_size - 2 * i)) & mask_clamp ;

        if exact && !v.simd_eq(w).to_bitmask() & mask == 0 {
            return i as isize;
        }
        else if !exact && mask32x32::from_bitmask(mask).select(v ^ w, ZERO).as_array().iter().map(|s| s.count_ones()).sum::<u32>() == 2 {
            return i as isize;
        }
    }
    -1
}

fn get_match_value(rows: &[u32], row_count: usize, cols: &[u32], col_count: usize, exact: bool) -> isize {
    let row_val = get_match_position(rows, row_count, exact);
    if row_val != -1 {
        return 100 * row_val;
    }
    get_match_position(cols, col_count, exact)
}

fn main() {
    let file = MmapFile::from_input("day13.txt");
    let mut rows = [0; 32];
    let mut columns = [0; 32];
    let mut x = 0;
    let mut y = 0;
    let mut had_newline = false;
    let mut mirror_end = false;
    let mut total = 0;
    let mut total_part2 = 0;

    for b in file.bytes() {
        let byte = b.unwrap();

        match byte {
            b'\n' => if had_newline {
                    mirror_end = true;
                }
                else {
                    y += 1;
                    had_newline = true;
                },
            b'#' => {
                if had_newline {
                    x = 0;
                }
                rows[y] |= 1 << x;
                columns[x] |= 1 << y;
                x += 1;
                had_newline = false;
            },
            _ => {
                if had_newline {
                    x = 0;
                }
                x += 1;
                had_newline = false;
            }
        }

        if mirror_end {
            total += get_match_value(&rows, y, &columns, x, true);
            total_part2 += get_match_value(&rows, y, &columns, x, false);

            had_newline = false;
            mirror_end = false;
            x = 0;
            y = 0;
            rows.fill(0);
            columns.fill(0);
        }
    }
    total += get_match_value(&rows, y, &columns, x, true);
    total_part2 += get_match_value(&rows, y, &columns, x, false);
    println!("{}\n{}", total, total_part2);
}
