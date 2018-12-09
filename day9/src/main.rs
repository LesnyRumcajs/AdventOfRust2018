#[derive(Clone)]
#[derive(Debug)]
struct Player {
    points: i32
}

struct Game {
    marbles: Vec<i32>,
    players: Vec<Player>,
    current_player_index: usize,
    current_marble_index: usize
}

fn log(msg: String) {
    println!("{}", msg);
}

impl Game {
    fn put(&mut self, marble: i32) {
        log(format!("Player #{} puts {}", self.current_player_index + 1, marble));

        if marble % 23 == 0 {

            let mut new_marble_index: i32 = self.current_marble_index as i32 - 7;
            if new_marble_index < 0 {
                new_marble_index = self.marbles.len() as i32 + new_marble_index;
            }
            let bonus = self.marbles[new_marble_index as usize];
            self.players[self.current_player_index as usize].points += marble + bonus;
            self.marbles.remove(new_marble_index as usize);
            self.current_marble_index = new_marble_index as usize;

        } else {
            let new_marble_index = match self.current_marble_index + 2 > self.marbles.len() {
                true => 1,
                _ => self.current_marble_index + 2
            };

            self.marbles.insert(new_marble_index, marble);
            self.current_marble_index = new_marble_index;
        }

        log(format!("Current marbles: {:?}", self.marbles));
        log(format!("Current stats: {:?}", self.players));
        self.current_player_index = (self.current_player_index + 1) % self.players.len();
    }
}

fn main() {
    const LAST_MARBLE:i32 = 7103500;
    const PLAYERS: usize = 479;

    let mut game = Game{marbles: vec![0],
                    players: vec![Player{points: 0}; PLAYERS],
                    current_player_index: 0,
                    current_marble_index: 0
    };

    for marble in 1..LAST_MARBLE+1 {
        if marble % 1000 == 0 {
            println!("Reached: {}/{}", marble, LAST_MARBLE);
        }
        game.put(marble);
    }

    println!("Result: {}",game.players.iter().max_by(|&x,&y| x.points.cmp(&y.points)).unwrap().points);
}
