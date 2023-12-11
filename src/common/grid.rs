use std::io::{Bytes, Read};
use std::ops::Index;
use memmap2::Mmap;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    data: Mmap
}

impl Grid {
    pub fn new(data: Mmap) -> Self {
        let mut width = 0;

        while data[width] != b'\n' {
            width = width + 1;
        }
        let height = data.len() / (width + 1);

        Self {
            data,
            width,
            height
        }
    }

    pub fn find_first(&self, find: u8) -> (usize, usize) {
        let index = self.data.iter().position(|&b| b == find).unwrap();
        (index % (self.width + 1), index / (self.width + 1))
    }

    pub fn bytes(&self) -> Bytes<&[u8]>{
        self.data.bytes()
    }
}

impl Index<[usize; 2]> for Grid {
    type Output = u8;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[1]* (self.width + 1) + index[1]]
    }
}