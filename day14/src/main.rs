struct Elf {
    recipe_index: usize
}

impl Elf {
    fn go(&mut self, scoreboard: &Vec<i32>) {
        self.recipe_index = (1 + self.recipe_index as i32 + scoreboard[self.recipe_index]) as usize % scoreboard.len();
    }
}

fn main() {
    score_check();
    sequence_check();
}

fn score_check() {
    const INPUT: usize = 513401 + 10;

    let mut scoreboard = vec![3,7];
    scoreboard.reserve(INPUT);

    let mut elf1 = Elf {recipe_index: 0};
    let mut elf2 = Elf {recipe_index: 1};

    while scoreboard.len() < INPUT {
        let mut new_recipes = create_new_recipes(scoreboard[elf1.recipe_index], scoreboard[elf2.recipe_index]);
        scoreboard.append(&mut new_recipes);

        elf1.go(&scoreboard);
        elf2.go(&scoreboard);

    }

    for el in scoreboard.len() - 10..scoreboard.len() {
        print!("{}", scoreboard[el]);
    }
    println!()
}

fn sequence_check() -> () {
    let input: Vec<i32> = vec![5, 1, 3, 4, 0, 1];
    let mut scoreboard = vec![3, 7];
    scoreboard.reserve(50000000);
    let mut elf1 = Elf { recipe_index: 0 };
    let mut elf2 = Elf { recipe_index: 1 };
    loop {
        let mut new_recipes = create_new_recipes(scoreboard[elf1.recipe_index], scoreboard[elf2.recipe_index]);
        scoreboard.append(&mut new_recipes);

        elf1.go(&scoreboard);
        elf2.go(&scoreboard);

        if scoreboard.len() > input.len() {
            let last_window = &scoreboard[scoreboard.len() - input.len()..];
            let almost_last_window = &scoreboard[scoreboard.len() - input.len() - 1..scoreboard.len() - 1];
            if last_window == &input[..] {
                println!("{}", scoreboard.len() - input.len());
                break;
            } else if almost_last_window == &input[..] {
                println!("{}", scoreboard.len() - input.len() - 1);
                break;
            }
        }
    }
}

fn create_new_recipes(first:i32, second:i32) -> Vec<i32> {
    let mut sum = first + second;

    let mut digits: Vec<i32> = Vec::new();
    while sum > 9 {
        digits.push(sum % 10);
        sum /= 10;
    }

    digits.push(sum);
    digits.reverse();
    digits
}
