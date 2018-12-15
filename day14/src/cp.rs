struct Elf {
    recipe_index: usize
}

impl Elf {
    fn go(&mut self, scoreboard: &Vec<i32>) {
        self.recipe_index = (1 + self.recipe_index as i32 + scoreboard[self.recipe_index]) as usize % scoreboard.len();
    }
}

fn main() {
    const INPUT: usize = 513401 + 10;

    let mut scoreboard = vec![3,7];
    scoreboard.reserve(INPUT);

    let mut elf1 = Elf {recipe_index: 0};
    let mut elf2 = Elf {recipe_index: 1};

    while scoreboard.len() < INPUT {
//        println!("Current index: [{}][{}]", elf1.recipe_index, elf2.recipe_index);
        let mut new_recipes = create_new_recipes(scoreboard[elf1.recipe_index], scoreboard[elf2.recipe_index]);
        scoreboard.append(&mut new_recipes);

        elf1.go(&scoreboard);
        elf2.go(&scoreboard);

    }

    for el in scoreboard.len() - 10..scoreboard.len() {
        print!("{}", scoreboard[el]);
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
