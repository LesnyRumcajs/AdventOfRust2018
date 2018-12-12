extern crate regex;

use std::io::{BufReader,BufRead};
use std::fs::File;
use regex::Regex;

struct Rule {
    dep: Vec<bool>,
    output: bool
}

fn main() {
    let (mut pots, rules) = load_data("res/test");
    let mut first_index = 0;

    let mut next_generation: Vec<bool> = Vec::new();
    const GENERATIONS: usize = 20;
    for generation in 1..GENERATIONS + 1 {
        for (no, pot) in pots.iter().enumerate() {
//            let window: Vec<bool> = match no {
//                0 => vec![]
//            }
            for rule in rules {

            }
        }
    }
}

fn load_data(filename: &str) -> (Vec<bool>, Vec<Rule>) {
    let f = File::open(filename).expect("input file not found");
    let f = BufReader::new(f);

    let init_re = Regex::new(r"^initial state: ([#\.]+)$").unwrap();
    let rule_re = Regex::new(r"^([#\.]+) => ([#\.])$").unwrap();

    let mut initial_state: Vec<bool>= Vec::new();
    let mut rules: Vec<Rule> = Vec::new();


    for (no, line) in f.lines().enumerate() {
        match no {
            0 => initial_state = init_re.captures(&line.unwrap()).unwrap()[1].chars().map(|x| contains_plant(x)).collect(),
            1 => (),
            _ => match rule_re.captures(&line.unwrap()) {
                Some(caps) => rules.push(Rule{
                    dep: caps[1].chars().map(|x| contains_plant(x)).collect(),
                    output: caps[2].chars().nth(0).map(|x| contains_plant(x)).unwrap()
                }),
                None => panic!("Messy input!")
            }
        }
    }

    (initial_state, rules)
}

fn contains_plant(symbol: char) -> bool {
    symbol.eq(&'#')
}
