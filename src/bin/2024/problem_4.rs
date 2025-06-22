use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Location(String);

#[derive(Clone)]
struct Graph {
    map: HashMap<Location, Vec<Location>>,
}

fn read_file(file_path: &str) -> Graph {
    let contents = std::fs::read_to_string(file_path).expect("Couldn't read the supplied file");

    let mut location_map: HashMap<Location, Vec<Location>> = HashMap::new();

    for location_line in contents.trim().split("\n") {
        let mut it = location_line.split(" ");
        let (left, _, right) = (it.next().unwrap(), it.next(), it.next().unwrap());
        let left = Location(left.to_string());
        let right = Location(right.to_string());

        location_map
            .entry(left.clone())
            .or_default()
            .push(right.clone());

        location_map.entry(right).or_default().push(left);
    }

    Graph { map: location_map }
}

fn part1(graph: Graph) -> usize {
    let mut location_map: HashMap<Location, usize> = HashMap::new();
    for location in graph.map.into_iter() {
        location_map.insert(location.0, 0);
        for neighbour in location.1 {
            location_map.insert(neighbour, 0);
        }
    }
    location_map.len()
}

fn part2(graph: Graph) -> usize {
    let mut location_list = vec![Location(String::from("STT"))];
    let mut new_location_list = Vec::new();
    let mut location_map: HashMap<Location, usize> = HashMap::new();
    let mut depth: i32 = 3;

    while depth >= 0 {
        for location in location_list {
            location_map.insert(location.clone(), 0);

            for neighbour in graph.map.get(&location).unwrap() {
                new_location_list.push(neighbour.clone());
            }
        }
        location_list = new_location_list.clone();
        depth -= 1;
    }
    location_map.len()
}

fn part3(graph: Graph) -> usize {
    let mut location_list = vec![Location(String::from("STT"))];
    let mut location_map: HashMap<Location, usize> = HashMap::new();
    let total_unique_locations = graph.map.len();
    let mut depth = 0;

    while location_map.len() != total_unique_locations {
        let mut new_location_list = Vec::new();

        for location in location_list {
            if !location_map.contains_key(&location) {
                location_map.insert(location.clone(), depth);
            }

            for neighbour in graph.map.get(&location).unwrap() {
                if !location_map.contains_key(neighbour) {
                    new_location_list.push(neighbour.clone());
                }
            }
        }

        depth += 1;
        location_list = new_location_list;
    }

    location_map.iter().map(|x| x.1).sum()
}

fn main() {
    let file_data = read_file("./input_4.txt");
    println!("Output of part 1: {}", part1(file_data.clone())); // Should print- 47
    println!("Output of part 2: {}", part2(file_data.clone())); // Should print- 22
    println!("Output of part 3: {}", part3(file_data.clone())); // Should print- 162
}

#[cfg(test)]
mod problem4_tests {
    use super::*;
    const INPUT: [&str; 6] = [
        "ADB <-> XYZ",
        "STT <-> NYC",
        "PLD <-> XYZ",
        "ADB <-> NYC",
        "JLI <-> NYC",
        "PTO <-> ADB",
    ];

    fn generate_test_graph() -> Graph {
        let mut location_map: HashMap<Location, Vec<Location>> = HashMap::new();
        for line in INPUT {
            let mut it = line.split(" ");
            let (left, _, right) = (it.next().unwrap(), it.next(), it.next().unwrap());
            let left = Location(left.to_string());
            let right = Location(right.to_string());

            location_map
                .entry(left.clone())
                .or_default()
                .push(right.clone());

            location_map.entry(right).or_default().push(left);
        }
        Graph { map: location_map }
    }

    #[test]
    fn test_part1() {
        let graph = generate_test_graph();
        assert_eq!(part1(graph), 7);
    }

    #[test]
    fn test_part2() {
        let graph = generate_test_graph();
        assert_eq!(part2(graph), 6);
    }

    #[test]
    fn test_part3() {
        let graph = generate_test_graph();
        assert_eq!(part3(graph), 15);
    }
}
