extern crate regex;
//#[macro_use] extern crate lazy_static;
use regex::Regex;
use regex::Captures;
use std::io::{self, BufRead};
use std::collections::HashMap;

struct Guard {
    id: String,
    sleep: u32,
    minutes_freq: HashMap<u32, u32>
}

struct SleepState {
    id: String,
    hour_start: u32,
    minutes_start: u32,
    sleep: u32,
    freq: HashMap<u32, u32>
}

fn main() {
    let lines = load_data();

    let shift_change = Regex::new(r"\[\d+-\d+-\d+ (\d+):(\d+)\] Guard #(\d+)").unwrap(); 
    let sleep_start = Regex::new(r"\[\d+-\d+-\d+ (\d+):(\d+)\] falls asleep").unwrap();
    let sleep_stop = Regex::new(r"\[\d+-\d+-\d+ (\d+):(\d+)\] wakes up").unwrap();

    let mut guards: Vec<Guard> = Vec::new();
    let first_id =  match shift_change.captures(&lines[0]) {
        Some(caps) => caps[3].to_string(),
        None => panic!()
    };

    let mut state = SleepState {hour_start: 0, minutes_start: 0, sleep: 0, freq: HashMap::new(), id: first_id};
    for line in lines.iter() {
        match shift_change.captures(line) {
            Some(caps) => {
                let id = caps[3].to_string();
                if state.id != id {
                    handle_shift_change(&mut guards, &mut state);

                    state = SleepState {hour_start: 0, minutes_start: 0, sleep: 0, freq: HashMap::new(), id: id};
                }
            }
            None => ()
        }
        match sleep_start.captures(line) {
            Some(caps) => {
                handle_sleep_start(&mut state, caps);
            }
            None => ()
        }
        match sleep_stop.captures(line) {
            Some(caps) => {
                handle_sleep_stop(&mut state, caps);
            }
            None => ()
        }
    }
    handle_shift_change(&mut guards, &mut state);

    // #1
    {
        let sleepy_guard = guards.iter().max_by(|x,y| {
            x.sleep.cmp(&y.sleep)}).unwrap();

        let max_min = get_max_minutes(&sleepy_guard);
        println!("{} * {}", sleepy_guard.id, max_min);
    }

    // # 2
    {
        let sleepy_guard = guards.iter().max_by(|x, y| {
            x.minutes_freq.values().max().cmp(&y.minutes_freq.values().max())
        }).unwrap();

        let max_min = get_max_minutes(&sleepy_guard);
        println!("{} * {}", sleepy_guard.id, max_min);
    }
}

fn handle_shift_change(guards: &mut Vec<Guard>, state: &mut SleepState) {
    let mut should_add = false;
    match guards.iter_mut().find(|x| x.id == state.id) {
        Some(x) => {
            x.sleep += state.sleep;
            for (k, v) in &state.freq {
                let entry = x.minutes_freq.entry(*k).or_insert(0);
                *entry += v;
            }
        },
        None => {
            should_add = true;
        }
    }
    if should_add {
        guards.push(Guard { id: state.id.clone(), sleep: state.sleep, minutes_freq: state.freq.clone() });
    }
}

fn handle_sleep_stop(state: &mut SleepState, caps: Captures) {
    let hour: u32 = caps[1].parse().unwrap();
    let min: u32 = caps[2].parse().unwrap();
    let sleep_time_stop = hour * 60 + min;
    let sleep_time_start = state.hour_start * 60 + state.minutes_start;
    state.sleep += sleep_time_stop - sleep_time_start;
    let mut local_sleep = sleep_time_stop - sleep_time_start;
    let mut minute = state.minutes_start;
    while local_sleep > 0 {
        let entry = state.freq.entry(minute).or_insert(0);
        *entry += 1;
        minute = (minute + 1) % 60;
        local_sleep -= 1;
    }
    state.minutes_start = 0;
    state.hour_start = 0;
}

fn handle_sleep_start(state: &mut SleepState, caps: Captures) {
    let hour: u32 = caps[1].parse().unwrap();
    let min: u32 = caps[2].parse().unwrap();
    state.hour_start = hour;
    state.minutes_start = min;
}

fn get_max_minutes(sleepy_guard: &&Guard) -> u32 {
    let mut max_min = 0;
    let mut max_min_count = 0;
    for (k, v) in &sleepy_guard.minutes_freq {
        if *v > max_min_count {
            max_min = *k;
            max_min_count = *v;
        }
    }
    max_min
}

fn load_data() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = match read_all_lines(stdin.lock()) {
        Err(err) => {
            println!("Failed to read input: {}", err);
            ::std::process::exit(1);
        }
        Ok(data) => data,
    };
    lines.sort();
    lines
}

fn read_all_lines<R: BufRead>(reader: R) -> Result<Vec<String>, io::Error> {
    reader.lines().collect()
}
