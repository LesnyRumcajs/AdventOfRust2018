use std::fs::File;
use std::io::Read;

fn main() {
    let filename = "input";
    println!("Reading file: {}", filename);

    let mut f = File::open(filename).expect("File not found!");
    let mut data = String::new();

    f.read_to_string(&mut data).expect("Error reading file");

    println!("Data:\n{}", data);
    println!("Data len:\n{}", data.len() - 1);

    let mut sizes: Vec<usize> = Vec::new();

    for polymer_type in b'A'..b'Z' + 1  {
        let mut bytes = data.clone().into_bytes();
        bytes.retain(|&x| x != polymer_type && x != polymer_type.to_ascii_lowercase());
        loop {
            let length_before_reduce = bytes.len();
            for i in 1..bytes.len() {
                let first = bytes[i];
                let second = bytes[i - 1];

                if are_different_polarity(first as char, second as char) && are_same_type(first as char, second as char) {
                    bytes[i] = b'#';
                    bytes[i - 1] = b'#';
                }
            }

            bytes.retain(|&x| x != b'#');


            if length_before_reduce == bytes.len() {
                break;
            }
        }

        sizes.push(bytes.len());
    }

    println!("Shortest polymer size: {}", sizes.iter().min().expect(":("));
}

fn are_different_polarity(first: char, second: char) -> bool {
    first.is_lowercase() && second.is_uppercase() || first.is_uppercase() && second.is_lowercase()
}

fn are_same_type(first: char, second: char) -> bool {
    first.to_ascii_lowercase() == second.to_ascii_lowercase()
}