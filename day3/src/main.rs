extern crate regex;
#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::io::{self, BufRead};
struct Fabric {
    id: String,
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

const DIM: usize = 1000;
fn main() {
    let stdin = io::stdin();
    let lines = match read_all_lines(stdin.lock()) {
        Err(err) => {
            println!("Failed to read input: {}", err);
            ::std::process::exit(1);
        }
        Ok(data) => data,
    };

    let mut whole_fabric: [[usize; DIM]; DIM] = [[0; DIM]; DIM];
    for line in lines.iter() {
        let patch = patch_from_line(line); 

        for column in patch.y..patch.y+patch.height {
            for row in patch.x..patch.x+patch.width {
                whole_fabric[row][column] += 1;
            }
        }
    }

    println!("Overlapping sum: {}", sum_overlapping(&mut whole_fabric));

    for line in lines.iter() {
        let patch = patch_from_line(line); 
        let mut good = 0;
        for column in patch.y..patch.y+patch.height {
            for row in patch.x..patch.x+patch.width {
                if whole_fabric[row][column] == 1 {
                    good += 1;
                }
            }
        }
        if good == patch.width * patch.height {
            println!("good id: {}", patch.id);
        }
    }
}

fn read_all_lines<R: BufRead>(reader: R) -> Result<Vec<String>, io::Error> {
    reader.lines().collect()
}

fn patch_from_line(line: &str) -> Fabric {
    lazy_static! {
        static ref re: Regex = Regex::new(r"^#(\d+)\s*@\s*(\d+),(\d+):\s*(\d+)x(\d+)$").unwrap();
    }
        let caps = re.captures(line).unwrap(); 
        Fabric {id: caps[1].parse().unwrap(),
            x: caps[2].parse().unwrap(),
            y: caps[3].parse().unwrap(),
            width: caps[4].parse().unwrap(),
            height: caps[5].parse().unwrap()
        }
}
fn sum_overlapping(fabric: &mut [[usize; DIM]; DIM]) -> u32 {
    let mut sum = 0;
    for column in 0..DIM {
        for row in 0..DIM {
            if fabric[row][column] > 1 {
                sum += 1;
            }
        }
    }
    sum
}
