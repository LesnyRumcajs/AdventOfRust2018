extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

struct Star {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32
}

impl Star {
    fn go(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
    }
}

fn main() {
    let mut stars = load_stars("res/input");

    // loop the input while the stars are not converged
    let mut seconds = 0;
    while !are_converged(&stars) {
        stars.iter_mut().for_each(|star| star.go());
        seconds += 1;
    }

    println!("Reached convergence after {} seconds!", seconds);
    print_stars(&stars);
}

fn print_stars(stars: &Vec<Star>) {
    let left_most = stars.iter().min_by(|lhs, rhs| lhs.x.cmp(&rhs.x)).unwrap().x;
//    println!("Left most: {}", left_most);
    let right_most = stars.iter().max_by(|lhs, rhs| lhs.x.cmp(&rhs.x)).unwrap().x;
//    println!("Right most: {}", right_most);

    let top_most = stars.iter().max_by(|lhs, rhs| lhs.y.cmp(&rhs.y)).unwrap().y;
//    println!("Top most: {}", top_most);
    let bottom_most = stars.iter().min_by(|lhs, rhs| lhs.y.cmp(&rhs.y)).unwrap().y;
//    println!("Bottom most: {}", bottom_most);

    let x_offset = left_most;
    let y_offset = bottom_most;

//    println!("x offset: {}\ny offset: {}", x_offset, y_offset);

    let width: usize = (right_most - x_offset + 1) as usize;
    let height: usize = (top_most - y_offset + 1) as usize;

//    println!("Width: {}\nHeight: {}", width, height);

    let grid = create_grid(width, height);

    for star in stars.iter() {
//        println!("Adding [x,y] : {}:{}", star.x, star.y);
        grid[(star.y - y_offset) as usize][(star.x - x_offset)as usize] = '*';
    }

    for row in 0..height {
        println!("{:?}", grid[row]);
    }

}

fn create_grid(width: usize, height: usize) -> &mut [&mut [char]] {
    let mut grid_raw = vec![' '; width * height];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
    let grid: &mut [&mut [_]] = grid_base.as_mut_slice();
    grid
}

fn are_converged(stars: &Vec<Star>) -> bool {
    let top_most = stars.iter().max_by(|lhs, rhs| lhs.y.cmp(&rhs.y)).unwrap();
    let bottom_most = stars.iter().min_by(|lhs, rhs| lhs.y.cmp(&rhs.y)).unwrap();

    top_most.y - bottom_most.y < 10
}

fn load_stars(filename: &str) -> Vec<Star> {
    let f = File::open(filename).expect("input file not found");
    let f = BufReader::new(f);

    let re = Regex::new(r"^position=<\s*([-]?\d+),\s*([-]?\d+)> velocity=<\s*([-]?\d+),\s*([-]?\d+)>$").unwrap();
    let mut stars: Vec<Star> = Vec::new();
    for line in f.lines() {
        match re.captures(&line.unwrap()) {
            Some(caps) => stars.push(Star { x: caps[1].parse().unwrap(),
                                                  y: caps[2].parse().unwrap(),
                                                  vel_x: caps[3].parse().unwrap(),
                                                  vel_y: caps[4].parse().unwrap()}),
            None => panic!("Messy input!")
        }
    }

    stars
}
