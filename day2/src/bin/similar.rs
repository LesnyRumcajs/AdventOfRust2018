use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = match read_all_lines(stdin.lock()) {
        Err(err) => {
            println!("Failed to read input: {}", err);
            ::std::process::exit(1);
        }
        Ok(result) => result,
    };

    for line in lines.iter() {
        match get_similar(&line, &lines) {
            Err(_err) => (),  
            Ok(result) => {
                println!("{}", result);
                return
            }
        };
    }
}

fn read_all_lines<R: BufRead>(reader: R) -> Result<Vec<String>, io::Error> {
    reader.lines().collect()
}

fn get_similar(line: &str, lines: &Vec<String>) -> Result<String, io::Error> {
    for compare in lines.iter() {
        let mut mismatch_count = 0;
        let mut mismatch_char = 0;
        for (i, character) in compare.chars().enumerate() {
            if line.chars().nth(i).unwrap() != character {
                mismatch_count += 1;
                mismatch_char = i;
            }

        }
        if mismatch_count == 1 {
            let mut result = String::from(line);
            result.remove(mismatch_char);
            return Ok(result);
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::Other, "failed miserably"))
}
