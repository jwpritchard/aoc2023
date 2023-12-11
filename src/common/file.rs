use std::env;
use std::fs::File;
use memmap2::{Mmap, MmapOptions};

pub struct MmapFile {

}

impl MmapFile {
    pub fn from_args() -> Mmap {
        let file = File::open(env::args().nth(1).unwrap()).unwrap();
        unsafe { MmapOptions::new().map(&file).unwrap() }
    }

    pub fn from_examples(s: &str) -> Mmap {
        let file = File::open(format!("{}{}", "examples/", s)).unwrap();
        unsafe { MmapOptions::new().map(&file).unwrap() }
    }

    pub fn from_input(s: &str) -> Mmap {
        let file = File::open(format!("{}{}", "input/", s)).unwrap();
        unsafe { MmapOptions::new().map(&file).unwrap() }
    }
}