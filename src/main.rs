extern crate rayon;

use std::fs;
use rayon::prelude::*;
use std::any::Any;

fn main() {
    let paths = fs::read_dir("/").unwrap();
    paths.into_iter().map(|entry| {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if !file_type.is_dir() {
                    println!("{:?}: {:?}", entry.path(), file_type);
                }
            }
        }
    }).count();
}
