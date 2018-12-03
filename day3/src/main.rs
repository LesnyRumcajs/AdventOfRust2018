extern crate regex;

use regex::Regex;
use std::io::{self, BufRead};
struct Fabric {
    id: String,
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

fn main() {
    let stdin = io::stdin();
    let lines = match read_all_lines(stdin.lock()) {
        Err(err) => {
            println!("Failed to read input: {}", err);
            ::std::process::exit(1);
        }
        Ok(data) => data,
    };

    const DIM: usize = 1000;
    let re = Regex::new(r"^#(\d+)\s*@\s*(\d+),(\d+):\s*(\d+)x(\d+)$").unwrap();
    let mut whole_fabric: [[u32; DIM]; DIM] = [[0; DIM]; DIM];
    for line in lines.iter() {
        let caps = re.captures(line).unwrap(); 
        let patch = Fabric {id: caps[1].parse().unwrap(),
            x: caps[2].parse().unwrap(),
            y: caps[3].parse().unwrap(),
            width: caps[4].parse().unwrap(),
            height: caps[5].parse().unwrap()
        };

        for column in patch.y..patch.y+patch.height {
            for row in patch.x..patch.x+patch.width {
                whole_fabric[row][column] += 1;
            }
        }
    }

    let mut sum = 0;
    for column in 0..DIM {
        for row in 0..DIM {
            if whole_fabric[row][column] > 1 {
                sum += 1;
            }
        }
    }
    println!("Overlapping sum: {}", sum);
}

fn read_all_lines<R: BufRead>(reader: R) -> Result<Vec<String>, io::Error> {
    reader.lines().collect()
}
