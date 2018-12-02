use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let mut twos = 0;
    let mut threes = 0;

    for line in stdin.lock().lines() {
        let (line_2, line_3) = get_occurences(&line.unwrap());
        twos += line_2;
        threes += line_3;
    }

    println!("Checksum is: {}", calculate_checksum(twos, threes));
}

fn calculate_checksum(twos: u32, threes: u32) -> u32 {
    twos * threes
}

fn get_occurences(line: &str) -> (u32, u32) {
    let mut frequency: HashMap<u8, u32> = HashMap::new();
    for character in line.bytes() {
        *frequency.entry(character).or_insert(0) += 1;
    }

    let mut twos: u32 = 0;
    let mut threes: u32 = 0;

    for (_, occurences) in &frequency {
        if *occurences == 2 && twos == 0 {
            twos += 1;
        } else if *occurences == 3 && threes == 0 {
            threes += 1;
        }
    }
    (twos, threes)
}
