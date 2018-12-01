use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    println!("Easy calibration: {}", easy_calibration());
    println!("Hard calibration: {}", hard_calibration());
    Ok(())
}

fn easy_calibration() -> i32 {
    let f = File::open("res/input").expect("input file not found");
    let f = BufReader::new(f);

    let mut sum: i32 = 0;
    for line in f.lines() {
        let change: i32 = line.unwrap().parse().expect("could not parse frequency change!");
        sum += change;
    }

    sum
}

fn hard_calibration() -> i32 {

    let mut frequencies = Vec::new();
    let mut sum: i32 = 0;
    frequencies.push(sum);
    loop {
        let f = File::open("res/input").expect("input file not found");
        let f = BufReader::new(f);
        for line in f.lines() {
            let change: i32 = line.unwrap().parse().expect("could not parse frequency change!");
            sum += change;

            if frequencies.contains(&sum) {
                return sum
            }
            frequencies.push(sum);
        }
    }
}
