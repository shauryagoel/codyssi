fn read_file(file_path: &str) -> Vec<i64> {
    let contents = std::fs::read_to_string(file_path).expect("Couldn't read the supplied file");

    contents
        .trim()
        .split("\n")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn part1(input: Vec<i64>) -> i64 {
    input.iter().sum()
}

fn part2(mut input: Vec<i64>, voucher_quantity: usize) -> i64 {
    input.sort();
    input.reverse();
    input[voucher_quantity..].iter().sum()
}

fn part3(input: Vec<i64>) -> i64 {
    let input_iter = input.iter();
    let a: i64 = input_iter.clone().step_by(2).sum();
    let b: i64 = input_iter.skip(1).step_by(2).sum();
    a - b
}

fn main() {
    let file_data = read_file("./input_1.txt");
    println!("Output of part 1: {}", part1(file_data.clone())); // Should print- 154971290
    println!("Output of part 2: {}", part2(file_data.clone(), 20)); // Should print- 135607900
    println!("Output of part 3: {}", part3(file_data.clone())); // Should print- -7563096
}

#[cfg(test)]
mod problem1_tests {
    use super::*;
    const INPUT: [i64; 6] = [912372, 283723, 294281, 592382, 721395, 91238];

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_vec()), 2895391);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_vec(), 2), 1261624);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(INPUT.to_vec()), 960705);
    }
}
