extern crate regex;
extern crate multiarray;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use regex::Regex;
use multiarray::*;
use std::collections::HashMap;

struct Point {
    id: String,
    x: i32,
    y: i32
}

#[derive(Clone, Debug)]
struct Field {
    id: String,
    distance: i32
}

impl Point {
    fn distance_to(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

fn main() {
    task1();
    task2();
}

fn task1() {
    const DIM: usize = 300;
    let points = load_points("input");

    let mut field : MultiArray<Field, Dim2>= Array2D::new([DIM, DIM], Field{id: String::from("."), distance: 999999});

    for x in 0..(DIM as i32){
        for y in 0..(DIM as i32) {
            for point in points.iter() {
                let distance = point.distance_to(x,y);
//                println!("distance: {}", distance);
                if field [[x as usize, y as usize]].distance == distance {
                    field [[x as usize,y as usize]].distance = -1;
                    field [[x as usize,y as usize]].id = String::from(".");
                } else if field [[x as usize,y as usize]].distance > distance {
                    field [[x as usize,y as usize]].distance = distance;
                    field [[x as usize,y as usize]].id = point.id.clone();
                }
            }

//            print!("{}", field[[x as usize,y as usize]].id);
        }
//        println!("");
    }

    let mut counts: HashMap<String, i32> = HashMap::new();
    for x in 0..(DIM as i32) {
        for y in 0..(DIM as i32) {
            let entry = counts.entry(field[[x as usize,y as usize]].id.clone()).or_insert(0);
            *entry += 1;
        }
    }

    let mut vals: Vec<i32> = counts.into_iter().map(|(_, val)| val).collect();
    vals.sort();

    println!("{:?}", vals);
}

fn task2() {
    const DIM: usize = 500;
    let points = load_points("input");

    let mut field : MultiArray<Field, Dim2>= Array2D::new([DIM, DIM], Field{id: String::from("."), distance: 999999});

    for x in 0..(DIM as i32){
        for y in 0..(DIM as i32) {
            let mut total_distance = 0;
            for point in points.iter() {
                total_distance += point.distance_to(x,y);
            }

            if total_distance < 10000 {
                field [[x as usize,y as usize]].distance = -1;
                field [[x as usize,y as usize]].id = String::from("X");
            } else {
                field [[x as usize,y as usize]].distance = -1;
                field [[x as usize,y as usize]].id = String::from(" ");
            }

//            print!("{}", field[[x as usize,y as usize]].id);
        }
//        println!("");
    }

    let mut sum = 0;
    for x in 0..(DIM as i32) {
        for y in 0..(DIM as i32) {
            if field [[x as usize,y as usize]].id == "X" {
                sum +=1;
            }
        }
    }

    println!("Task 2 largest area: {}", sum);
}

fn load_points(filename: &str) -> Vec<Point> {
    let f = File::open(filename).expect("input file not found");
    let f = BufReader::new(f);

    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    let mut points: Vec<Point> = Vec::new();
    let mut id = 0;
    for line in f.lines() {
        match re.captures(&line.unwrap()) {
            Some(caps) => points.push(Point { id: id.to_string(), x: caps[2].parse().unwrap(), y: caps[1].parse().unwrap() }),
            None => panic!("Messy input!")
        }
        id += 1;
    }

    points
}
