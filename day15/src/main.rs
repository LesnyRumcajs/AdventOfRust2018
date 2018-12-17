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

    fn is_alive(&self) -> bool {
        return self.hp > 0
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
    let mut battleground = load_input("res/test");
    print_battleground(&battleground);

    let mut units = get_units_from_battleground(&battleground);
    units.sort_by_key(|unit| (unit.position.y, unit.position.y));
    let units = units;

//    let some_unit = units[0];
//    println!("{}:{}", some_unit.position.x, some_unit.position.y);

//    battleground.swap(0,1);

//    units.sort_by_key(|x| x.);
//    if let Field::Warrior(warrior) = units[0] {
//
//    }
    print_battleground(&battleground);
}

fn get_units_from_battleground(battleground: &Battleground) -> Vec<&Unit>{
    battleground.iter().flatten().into_iter().filter(|x| match x {
        Field::Warrior(warrior) => warrior.is_alive(),
        _ => false
    }).map(|x| match x {
        Field::Warrior(warrior) => warrior,
        _ => panic!("oh no no no!")
    }).collect()
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

fn get_faction_adjacent_positions(faction: Faction, battleground: &Battleground) -> Vec<Pos> {
    let positions: Vec<Pos> = Vec::new();
    for row in battleground.iter() {
        for x in row.iter() {
//            match x {
//                Field::Warrior(warrior) => {
//                    if
//                }
//            }
        }
    }
    positions
}