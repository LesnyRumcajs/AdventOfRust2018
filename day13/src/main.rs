use std::fs::File;
use std::io::{BufReader,BufRead};

enum Turn {
    Left,
    Straight,
    Right
}

struct Cart {
    vel_x: i32,
    vel_y: i32,
    x: i32,
    y: i32,
    next_turn: Turn
}

impl Cart {
    fn from(symbol: char, x: i32, y: i32) -> Cart {
        let (vel_x, vel_y) = match symbol {
            '<' => (-1,0),
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            _ => panic!("Invalid cart symbol!")
        };

        Cart {
            vel_x, vel_y, x, y, next_turn: Turn::Left
        }
    }
}

fn main() {
    load_data("res/test");
    println!("Hello, Mine Cart Madness!");
}

fn load_data(filename: &str) -> () {
    let f = File::open(filename).expect("input file not found");
    let f = BufReader::new(f);

    let mut carts: Vec<Cart> = Vec::new();

    for (y, line) in f.lines().enumerate() {
        for (x, symbol) in line.unwrap().chars().enumerate() {
            if is_cart(symbol) {
                println!("Found cart [{}] at: {}:{}", symbol, x, y );
                carts.push(Cart::from(symbol, x as i32, y as i32));
            }
        }
    }

    ()
}

fn is_cart(symbol: char) -> bool {
    symbol == '<' || symbol == '>' || symbol == '^' || symbol == 'v'
}
