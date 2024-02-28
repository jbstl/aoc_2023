use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(8);

/// Return a a tuple containing:
///   - The instructions as a vector of 0s and 1s
///   - A HashMap of the network of nodes
fn parse_input(input: &str) -> (Vec<u8>, HashMap<&str, (&str, &str)>) {
    let mut input = input.split("\n\n");
    let instructions = input.next().unwrap();
    let instructions = instructions
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .collect();

    let mut network = HashMap::new();
    let nodes = input.next().unwrap();
    let pattern = r"(?m)^(\w+)\s*=\s*\((\w+),\s*(\w+)\)$";
    let re = Regex::new(pattern).unwrap();
    for (_, [src, left, right]) in re.captures_iter(nodes).map(|x| x.extract()) {
        network.insert(src, (left, right));
    }

    (instructions, network)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (instructions, network) = parse_input(input);
    let mut steps = 0;
    let goal = "ZZZ";
    let instruction_count = instructions.len();
    let mut current_node = String::from("AAA");
    while current_node != goal {
        let instruction = instructions[steps % instruction_count];
        steps += 1;
        let node = match instruction {
            0 => network.get(current_node.as_str()).unwrap().0,
            1 => network.get(current_node.as_str()).unwrap().1,
            _ => unreachable!(),
        };
        current_node.clear();
        current_node.push_str(node);
    }
    Some(steps)
}

/// Calculate Greatest Common Divisor using Euclid's algorithm
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Calculate the lowest common multiple based on the greatest common divisor
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (instructions, network) = parse_input(input);
    let instruction_count = instructions.len();

    let starting_nodes = network
        .iter()
        .filter(|(key, _val)| key.ends_with('A'))
        .map(|(key, _val)| String::from(*key))
        .collect::<Vec<String>>();
    let mut steps_to_end_in_z = Vec::new();

    let mut current_node = String::new();
    let mut steps;

    // Create a vector with the number of steps it takes each starting node to end in Z.
    for starting_node in starting_nodes.iter() {
        current_node.clear();
        current_node.push_str(starting_node);
        steps = 0;
        while !current_node.ends_with('Z') {
            let node = match instructions[steps % instruction_count] {
                0 => network.get(current_node.as_str()).unwrap().0,
                1 => network.get(current_node.as_str()).unwrap().1,
                _ => unreachable!(),
            };
            steps += 1;
            current_node.clear();
            current_node.push_str(node);
        }
        steps_to_end_in_z.push(steps);
    }

    // To find the total number of steps, take the gcd for all the step counts to end at a Z
    steps = steps_to_end_in_z.into_iter().fold(1, |acc, n| lcm(acc, n));

    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
