advent_of_code::solution!(9);
use itertools::Itertools;
type Num = i32;

pub fn parse_input(input: &str) -> Vec<Vec<Num>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| {
                    num.parse::<Num>()
                        .expect("Unable to parse integer from input")
                })
                .collect_vec()
        })
        .collect_vec()
}

pub fn get_diff_sequence(input: &[Num]) -> Vec<Num> {
    let mut diffs = Vec::new();
    for (a, b) in input.iter().tuple_windows() {
        diffs.push(b - a);
    }
    diffs
}

pub fn predict_next_value(input: &[Num]) -> Num {
    if input.iter().filter(|num| num != &&0).count() == 0 {
        0
    } else {
        let diffs = get_diff_sequence(input);
        let last_val = input[input.len() - 1];
        last_val + predict_next_value(&diffs)
    }
}

pub fn predict_previous_value(input: &[Num]) -> Num {
    if input.iter().filter(|num| num != &&0).count() == 0 {
        0
    } else {
        let diffs = get_diff_sequence(input);
        let first_val = input[0];
        first_val - predict_previous_value(&diffs)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse_input(input);
    let total = input.iter().map(|vec| predict_next_value(vec)).sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = parse_input(input);
    let total = input.iter().map(|vec| predict_previous_value(vec)).sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
