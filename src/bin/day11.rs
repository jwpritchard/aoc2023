use aoc2023::common::file::MmapFile;
use aoc2023::common::grid::Grid;

#[inline]
fn calculate_distance((a, b): (usize, usize), (c, d): (usize, usize)) -> i64 {
    (a as i64 - c as i64).abs() + (b as i64 - d as i64).abs()
}

fn main() {
    let mmap = MmapFile::from_args();
    let grid = Grid::new(mmap);

    let mut empty_row: [bool; 150] = [true; 150];
    let mut empty_column: [bool; 150] = [true; 150];
    let mut row_total: [usize; 150] = [0; 150];
    let mut column_total: [usize; 150] = [0; 150];
    let mut row_total_pt2: [usize; 150] = [0; 150];
    let mut column_total_pt2: [usize; 150] = [0; 150];
    let mut galaxies: Vec<(usize, usize)> = Vec::with_capacity(500);

    let (mut x, mut y) = (0, 0);

    for b in grid.bytes() {
        let byte = b.unwrap();

        if byte == b'.' {
            x = x + 1;
        }
        else if byte == b'#' {
            empty_row[y] = false;
            empty_column[x] = false;
            galaxies.push((x, y));
            x = x + 1
        }
        else {
            x = 0;
            y = y + 1
        }
    }

    let mut running_total = 0;
    let mut running_total_pt2 = 0;
    for i in 0..grid.width {
        running_total += if empty_column[i] { 1 } else { 0 };
        running_total_pt2 += if empty_column[i] { 999999 } else { 0 };
        row_total[i] = running_total;
        row_total_pt2[i] = running_total_pt2;
    }

    let mut running_total = 0;
    let mut running_total_pt2 = 0;
    for i in 0..grid.height {
        running_total += if empty_row[i] { 1 } else { 0 };
        running_total_pt2 += if empty_row[i] { 999999 } else { 0 };
        column_total[i] = running_total;
        column_total_pt2[i] = running_total_pt2;
    }

    let mut total = 0;
    let mut total_pt2 = 0;
    let galaxy_count = galaxies.len();

    for i in 0..galaxy_count {
        for j in (i + 1)..galaxy_count {
            let (a, b) = galaxies[i];
            let (c, d) = galaxies[j];
            total += calculate_distance((a + row_total[a], b + column_total[b]),(c + row_total[c], d + column_total[d]));
            total_pt2 += calculate_distance((a + row_total_pt2[a], b + column_total_pt2[b]),(c + row_total_pt2[c], d + column_total_pt2[d]));
        }
    }
    println!("{}\n{}", total, total_pt2);
}
