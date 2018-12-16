extern crate pathfinding;

//use pathfinding::grid::Grid;
//use pathfinding::prelude::bfs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt;

#[derive(PartialEq, PartialOrd)]
enum Field {
    Boundary,
    Free,
    Warrior(Unit)
}

#[derive(PartialEq, PartialOrd)]
enum Faction {
    Elf,
    Goblin
}

#[derive(PartialEq, PartialOrd)]
struct Unit {
    attack_power: i32,
    hp: i32,
    faction: Faction,
    position: Pos
}

impl Unit {
    fn new(faction: Faction, position: Pos) -> Unit {
        Unit {
            hp: 200,
            attack_power: 3,
            faction,
            position
        }
    }
}

type Battleground = Vec<Vec<Field>>;

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Field::Free => '.',
            Field::Boundary => '#',
            Field::Warrior(ref unit) => match unit.faction {
                Faction::Elf => 'E',
                Faction::Goblin => 'G'
            }
        };
        write!(f, "{}", printable)
    }
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    fn successors(&self) -> Vec<Pos> {
        let (x,y) = (self.x, self.y);
        vec![Pos{x: x+1,y}, Pos{x: x-1,y}, Pos{x,y: y+1}, Pos{x,y: y-1}]
    }

    fn new(x: usize, y: usize) -> Pos {
        Pos{x: x as i32, y: y as i32}
    }
}

fn main() {
    let battleground = load_input("res/input");
    print_battleground(&battleground);

    let units = get_units_from_battleground(&battleground);
}

fn get_units_from_battleground(battleground: &Battleground) {
    for row in battleground.iter() {
        for field in row.iter() {
            if let Field::Warrior(warrior) = field {
                println!("Found at {:?} ", warrior.position);
            }
        }
    }
}

fn load_input(filename: &str) -> Battleground {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
    f.lines().enumerate().map(|(y, line)| line.unwrap().chars().enumerate().map(|(x,ch)| match ch {
        'G' => Field::Warrior(Unit::new(Faction::Goblin, Pos::new(x,y))),
        'E' => Field::Warrior(Unit::new(Faction::Elf, Pos::new(x,y))),
        '#' => Field::Boundary,
        '.' => Field::Free,
        _ => panic!("Unsupported field type!")
    }).collect()).collect()
}

fn print_battleground(battleground: &Battleground) {
    for line in battleground.iter() {
        for field in line.iter() {
            print!("{}", field);
        }
        println!();
    }
}
