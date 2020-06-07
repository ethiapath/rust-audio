use std::fs::File;
use std::io::prelude::*;

fn main () {
    let mut file = File::create("sine.wav").unwrap();
    println!("Hello, world!");
}


