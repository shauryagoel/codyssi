use std::str::FromStr;

#[derive(Clone, Copy)]
enum Base {
    Base2,
    Base8,
    Base10,
    Base16,
}

impl FromStr for Base {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Base::Base2),
            "8" => Ok(Base::Base8),
            "10" => Ok(Base::Base10),
            "16" => Ok(Base::Base16),
            _ => Err(()),
        }
    }
}

impl From<Base> for usize {
    fn from(value: Base) -> Self {
        match value {
            Base::Base2 => 2,
            Base::Base8 => 8,
            Base::Base10 => 10,
            Base::Base16 => 16,
        }
    }
}

#[derive(Clone)]
struct DroneReading {
    reading: String,
    base: Base,
}

// Convert a string of "<READING> <BASE>" to `DroneReading` struct object
impl FromStr for DroneReading {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (reading, base) = s.split_once(" ").unwrap();
        Ok(DroneReading {
            reading: reading.to_string(),
            base: base.parse().unwrap(),
        })
    }
}

const BASE65_SYMBOLS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#";

// Convert string from any base to base 10
fn convert_to_base10(reading: &str, base: Base) -> usize {
    usize::from_str_radix(reading, usize::from(base) as u32).unwrap()
}

fn convert_from_base10_to_base65(mut reading: usize) -> String {
    // Create a mapping from position in base65 to actual character of base65
    let id2base65: Vec<&str> = BASE65_SYMBOLS
        .chars()
        .enumerate()
        .map(|(i, _)| &BASE65_SYMBOLS[i..(i + 1)])
        .collect();

    let mut output = String::new();
    while reading > 0 {
        let remainder = reading % 65;
        output.push_str(id2base65[remainder]);
        reading /= 65;
    }

    output.chars().rev().collect() // Reverse a String
}

fn read_file(file_path: &str) -> Vec<DroneReading> {
    let contents = std::fs::read_to_string(file_path).expect("Couldn't read the supplied file");

    contents
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(input: Vec<DroneReading>) -> usize {
    input
        .into_iter()
        .fold(0, |acc, x| acc + Into::<usize>::into(x.base))
}

fn part2(input: Vec<DroneReading>) -> usize {
    input
        .into_iter()
        .map(|x| convert_to_base10(&x.reading, x.base))
        .sum()
}

fn part3(input: Vec<DroneReading>) -> String {
    let sum = part2(input);
    convert_from_base10_to_base65(sum)
}

fn main() {
    let file_data = read_file("./input_3.txt");
    println!("Output of part 1: {}", part1(file_data.clone())); // Should print- 7244
    println!("Output of part 2: {}", part2(file_data.clone())); // Should print- 394357550460
    println!("Output of part 3: {}", part3(file_data.clone())); // Should print- 5Ev5eCt
}

#[cfg(test)]
mod problem3_tests {
    use super::*;
    const INPUT: [&str; 8] = [
        "100011101111110010101101110011 2",
        "83546306 10",
        "1106744474 8",
        "170209FD 16",
        "2557172641 8",
        "2B290C15 16",
        "279222446 10",
        "6541027340 8",
    ];

    #[test]
    fn test_part1() {
        let input = INPUT.map(|x| x.parse().unwrap());
        assert_eq!(part1(input.to_vec()), 78);
    }

    #[test]
    fn test_part2() {
        let input = INPUT.map(|x| x.parse().unwrap());
        assert_eq!(part2(input.to_vec()), 3487996082);
    }

    #[test]
    fn test_part3() {
        let input = INPUT.map(|x| x.parse().unwrap());
        assert_eq!(part3(input.to_vec()), "30PzDC");
    }
}
