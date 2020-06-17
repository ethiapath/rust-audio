use std::fs::File;
use std::io::prelude::*;

const SAMPLE_RATE: u16 = 44100;

fn make_sin (nsecs: u64, freq: f64) -> Vec<u8> {
    let nsamples = nsecs as usize * SAMPLE_RATE as usize;
    let mut buf = Vec::with_capacity(nsamples);
    for t in 0..nsamples {
        let s = f64::sin(2.0 * std::f64::consts::PI * t as f64 / (SAMPLE_RATE as f64));

        let s = f64::floor(255.0 * (0.5 * s + 0.5)) as u8;
        buf.push(s);
    }
    buf
}

fn main () {
    let mut file = File::create("sine.wav").unwrap();
    println!("Hello, world!");
    let buf = make_sin(3, 1000.0);
    file.write_all(&buf).unwrap();
}


