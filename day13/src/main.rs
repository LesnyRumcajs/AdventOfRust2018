use std::fs::File;
use std::io::{BufReader,BufRead};
use std::cell::RefCell;

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
    next_turn: Turn,
    good: bool
}

impl Cart {
    fn from(symbol: char, x: i32, y: i32) -> Cart {
        let (vel_x, vel_y) = match symbol {
            '<' => (-1,0),
            '^' => (0, -1),
            'v' => (0, 1),
            '>' => (1, 0),
            _ => panic!("Invalid cart symbol!")
        };

        Cart {
            vel_x, vel_y, x, y, next_turn: Turn::Left, good: true
        }
    }

    fn go(&mut self, lines: &Vec<String>) {
        if !self.good {
            return
        }

        self.x += self.vel_x;
        self.y += self.vel_y;

        let symbol = get_symbol_at(&lines, self.x, self.y);
        match symbol {
            '+' => match self.next_turn {
                Turn::Left => {
                    self.next_turn = Turn::Straight;
                    std::mem::swap(&mut self.vel_x, &mut self.vel_y);
                    self.vel_y *= -1;
                },
                Turn::Straight => self.next_turn = Turn::Right,
                Turn::Right => {
                    self.next_turn = Turn::Left;
                    std::mem::swap(&mut self.vel_x, &mut self.vel_y);
                    self.vel_x *= -1;
                }
            },
            '\\' => {
                std::mem::swap(&mut self.vel_x, &mut self.vel_y);
            },
            '/' => {
                std::mem::swap(&mut self.vel_x, &mut self.vel_y);
                self.vel_x *= -1;
                self.vel_y *= -1;
            }
            _ => ()
        }
    }
}

fn main() {
    let (lines, mut carts) = load_data("res/input");

    let mut should_continue = true;
    while should_continue {
        carts.sort_by_key(|cart| (cart.borrow().y, cart.borrow().x));

        for cart in carts.iter() {
            cart.borrow_mut().go(&lines);

            if check_for_collisions(&cart, &carts) {
                println!("Crash at: {}:{}", cart.borrow().x, cart.borrow().y);
            }
        }

        let left: Vec<&RefCell<Cart>> = carts.iter().filter(|cart| cart.borrow().good).collect();
        if left.iter().count() < 2 {
            println!("Finishing with cart at: {}:{}", left[0].borrow().x, left[0].borrow().y);
            should_continue = false;
        }
    }
}

fn check_for_collisions(cart: &RefCell<Cart>, carts: &Vec<RefCell<Cart>>) -> bool {
    let mut same_pos = 0;
    for candidate in carts.iter() {

        if candidate.as_ptr() != cart.as_ptr()
            && candidate.borrow().x == cart.borrow().x
            && candidate.borrow().y == cart.borrow().y
            && candidate.borrow().good && cart.borrow().good {
            same_pos += 1;
            candidate.borrow_mut().good = false;
            cart.borrow_mut().good = false;
        }
    }
    same_pos == 1
}

fn get_symbol_at(lines: &Vec<String>, x:i32, y:i32) -> char {
    lines[y as usize].chars().nth(x as usize).unwrap()
}

fn load_data(filename: &str) -> (Vec<String>, Vec<RefCell<Cart>>) {
    let f = File::open(filename).expect("input file not found");
    let f = BufReader::new(f);

    let lines:Vec<String> = f.lines().map(|l| l.expect("Failed to parse line!")).collect();
    let carts = get_carts_from_grid(&lines);

    (lines, carts)

}

fn get_carts_from_grid(lines: &Vec<String>) -> Vec<RefCell<Cart>> {
    let mut carts: Vec<RefCell<Cart>> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if is_cart(symbol) {
                carts.push(RefCell::new(Cart::from(symbol, x as i32, y as i32)));
            }
        }
    }
    carts
}

fn is_cart(symbol: char) -> bool {
    symbol == '<' || symbol == '>' || symbol == '^' || symbol == 'v'
}
