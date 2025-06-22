use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sensor {
    False,
    True,
}

impl FromStr for Sensor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FALSE" => Ok(Sensor::False),
            "TRUE" => Ok(Sensor::True),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum Op {
    And,
    Or,
}

// Define the operation of gates
fn perform_operation(input1: Sensor, input2: Sensor, operation: Op) -> Sensor {
    match operation {
        Op::And => match (input1, input2) {
            (Sensor::True, Sensor::True) => Sensor::True,
            _ => Sensor::False,
        },
        Op::Or => match (input1, input2) {
            (Sensor::False, Sensor::False) => Sensor::False,
            _ => Sensor::True,
        },
    }
}

fn read_file(file_path: &str) -> Vec<Sensor> {
    let contents = std::fs::read_to_string(file_path).expect("Couldn't read the supplied file");

    contents
        .trim()
        .split("\n")
        .map(|x| Sensor::from_str(x).unwrap())
        .collect()
}

fn part1(input: Vec<Sensor>) -> usize {
    input
        .into_iter()
        .enumerate()
        .filter(|x| x.1 == Sensor::True)
        .fold(0, |acc, x| acc + x.0 + 1) // Add one as indexing starts from 0 in enumerate
}

fn part2(input: Vec<Sensor>) -> usize {
    let it = input.into_iter();
    // Create the four iterators for both pairs of elements
    let it1 = it.clone().step_by(4);
    let it2 = it.clone().skip(1).step_by(4);
    let it3 = it.clone().skip(2).step_by(4);
    let it4 = it.clone().skip(3).step_by(4);

    let a = it1
        .zip(it2)
        .map(|(x, y)| perform_operation(x, y, Op::And))
        .filter(|&x| x == Sensor::True)
        .count();
    let b = it3
        .zip(it4)
        .map(|(x, y)| perform_operation(x, y, Op::Or))
        .filter(|&x| x == Sensor::True)
        .count();
    a + b
}

fn part3(mut input: Vec<Sensor>) -> usize {
    let mut total_count = 0_usize;

    while !input.is_empty() {
        total_count += input.iter().filter(|&&x| x == Sensor::True).count();

        let mut current_operation = Op::And;
        let mut next_layer = Vec::new();

        for chunk in input.chunks(2) {
            if chunk.len() == 2 {
                let new_output = perform_operation(chunk[0], chunk[1], current_operation);
                next_layer.push(new_output);
                // Invert the operator
                current_operation = match current_operation {
                    Op::And => Op::Or,
                    Op::Or => Op::And,
                }
            }
        }
        input = next_layer; // Make input behave like a queue but without inplace updates
    }
    total_count
}

fn main() {
    let file_data = read_file("./input_2.txt");
    println!("Output of part 1: {}", part1(file_data.clone())); // Should print- 65741
    println!("Output of part 2: {}", part2(file_data.clone())); // Should print- 125
    println!("Output of part 3: {}", part3(file_data.clone())); // Should print- 506
}

#[cfg(test)]
mod problem2_tests {
    use super::*;
    const INPUT: [Sensor; 8] = [
        Sensor::True,
        Sensor::False,
        Sensor::True,
        Sensor::False,
        Sensor::False,
        Sensor::False,
        Sensor::True,
        Sensor::True,
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_vec()), 19);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_vec()), 2);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(INPUT.to_vec()), 7);
    }
}
