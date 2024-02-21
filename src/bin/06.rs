use core::f64;
use std::u32;

advent_of_code::solution!(6);

// Parse the input and return a list of 2-tuples. The first number in the tuple is the time, the
// second is the distance needed to break the reacord.
fn parse_input_part_one(input: &str) -> Vec<(u32, u32)> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num_str| {
            num_str
                .parse::<u32>()
                .expect("Unable to parse number from file")
        })
        .collect::<Vec<u32>>();

    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num_str| {
            num_str
                .parse::<u32>()
                .expect("Unable to parse number from file")
        })
        .collect::<Vec<u32>>();
    std::iter::zip(time, distance).collect()
}
// Parse the input and return a list of 2-tuples. The first number in the tuple is the time, the
// second is the distance needed to break the reacord.
fn parse_input_part_two(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    (time, distance)
}

// Solves quadratic equations with real roots
fn solve_real_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let d = b.powi(2) - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        Some((root1, root2))
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        Some((root, root))
    } else {
        // D is negative and roots are complex. Not needed for this example.
        None
    }
}

// The distance the toy travels == hold_time * time_left
// == hold_time * (race_time - hold_time)
// ==  -hold_time^2 + (hold_time * race_time)
// ==  -hold_time^2 + (hold_time * race_time) = record_distance
// ==  -hold_time^2 + (hold_time * race_time) - record_distance = 0
// To find the hold_times that could've created the record distance we can solve
// the quadratic for the record distance.
fn get_new_record_hold_times(race_time: f64, record_distance: f64) -> Option<(u32, u32)> {
    let (hold_time_1, hold_time_2) =
        solve_real_quadratic(-1.0, race_time, record_distance * -1.0).unwrap();
    let lower_bound = if hold_time_1.fract() == 0.0 {
        hold_time_1 + 1.0
    } else {
        hold_time_1.ceil()
    };
    let upper_bound = if hold_time_2.fract() == 0.0 {
        hold_time_2 - 1.0
    } else {
        hold_time_2.floor()
    };
    Some((lower_bound as u32, upper_bound as u32))
}

pub fn part_one(input: &str) -> Option<usize> {
    let inputs = parse_input_part_one(input);
    let mut count = 1;
    for (race_time, record_distance) in inputs {
        let (lower_bound, upper_bound) =
            get_new_record_hold_times(f64::from(race_time), f64::from(record_distance)).unwrap();
        let range = (lower_bound..=upper_bound).count();
        count *= range;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (race_time, record_distance) = parse_input_part_two(input);
    let (lower_bound, upper_bound) =
        get_new_record_hold_times(race_time as f64, record_distance as f64).unwrap();
    let range = (lower_bound..=upper_bound).count();
    Some(range)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }

    #[test]
    fn test_parse_input_part_one() {
        let result = parse_input_part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, vec![(7, 9), (15, 40), (30, 200)]);
    }
}
