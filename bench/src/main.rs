extern crate romaji;

use std::fs;
use romaji::romaji::RomajiExt;
use std::io::prelude::*;

fn main() {
    let mut file = fs::File::open("./text/romaji.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents.to_katakana());
}
