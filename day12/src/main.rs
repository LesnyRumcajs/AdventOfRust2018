extern crate regex;

use std::io::{BufReader,BufRead};
use std::fs::File;
use regex::Regex;

struct Rule {
    dep: Vec<bool>,
    output: bool
}

const PRE: i32 = 20;

fn main() {
    let (mut pots, rules) = load_data("res/input");

    for _ in 0..PRE {
        pots.insert(0, false);
    }


    let mut next_generation: Vec<bool> = Vec::new();
    let mut prev_sum = 0;
    let mut prev_diff = 0;
    let mut same_diff_count = 0;

    const GENERATIONS: usize = 20;
    let mut generations_completed = 0;

    for generation in 1..GENERATIONS + 1 {
        for no in 0..pots.len() + 2 {

            let window: Vec<bool> = match no {
                0 => vec![false, false, *pots.get(no).unwrap_or(&false), *pots.get(no + 1).unwrap_or(&false), *pots.get(no + 2).unwrap_or(&false)],
                1 => vec![false, *pots.get(no - 1).unwrap_or(&false), pots[no], *pots.get(no + 1).unwrap_or(&false), *pots.get(no + 2).unwrap_or(&false)],
                _ => vec![*pots.get(no - 2).unwrap_or(&false), *pots.get(no - 1).unwrap_or(&false), *pots.get(no).unwrap_or(&false), *pots.get(no + 1).unwrap_or(&false), *pots.get(no + 2).unwrap_or(&false)],
            };

            let mut will_grow = false;
            for rule in rules.iter() {
                if rule.dep == window {
                    will_grow = rule.output;
                    break;
                }
            }

            next_generation.push(will_grow);
        }

        pots = next_generation;
        next_generation = Vec::new();

        generations_completed = generation;

        let sum = sum_pots(&mut pots);
        let diff = sum - prev_sum;

        if diff == prev_diff {
            same_diff_count += 1;
        } else {
            same_diff_count = 0;
        }

        if same_diff_count > 5 {
            break;
        }
        prev_sum = sum;
        prev_diff = diff;
    }

    if generations_completed == GENERATIONS {
        println!("{}", sum_pots(&mut pots));
    } else {
        println!("{}", prev_sum as usize + prev_diff as usize *(GENERATIONS - generations_completed as usize + 1) as usize);
    }

}

fn sum_pots(pots: &mut Vec<bool>) -> i32 {
    let mut sum = 0;
    for (no, pot) in pots.iter().enumerate() {
        if *pot {
            sum += no as i32 - PRE;
        }
    }
    sum
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
