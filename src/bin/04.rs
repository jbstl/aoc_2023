advent_of_code::solution!(4);

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parsers/day4.pest"]
struct CardsParser;

fn get_card_point_map(input: &str) -> Vec<u32> {
    let cards_parse = CardsParser::parse(Rule::cards, input)
        .unwrap()
        .next()
        .unwrap();
    let mut result = Vec::new();

    for card in cards_parse.into_inner() {
        if let Rule::card = card.as_rule() {
            let mut points = 0;
            let mut winning_nums = Vec::new();
            let mut my_nums = Vec::new();
            for section in card.into_inner() {
                match section.as_rule() {
                    Rule::winning_nums => {
                        for num in section.into_inner() {
                            let num = num
                                .as_str()
                                .parse::<u32>()
                                .expect("Unable to parse str to number");
                            winning_nums.push(num);
                        }
                    }
                    Rule::my_nums => {
                        for num in section.into_inner() {
                            let num = num
                                .as_str()
                                .parse::<u32>()
                                .expect("Unable to parse str to number");
                            my_nums.push(num);
                        }
                    }
                    _ => {}
                }
            }

            // Sort the vectors and compare the largest values at the end to find matching values
            // in constant time.
            winning_nums.sort();
            my_nums.sort();
            while !(winning_nums.is_empty() | my_nums.is_empty()) {
                let last_winning_index = winning_nums.len() - 1;
                let last_my_index = my_nums.len() - 1;
                let last_winning_number = winning_nums[last_winning_index];
                let last_my_number = my_nums[last_my_index];

                if last_winning_number > last_my_number {
                    let _ = winning_nums.pop();
                } else if last_my_number > last_winning_number {
                    let _ = my_nums.pop();
                } else {
                    points += 1;
                    let _ = my_nums.pop();
                    let _ = winning_nums.pop();
                }
            }
            result.push(points);
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let points = get_card_point_map(input)
        .iter()
        .map(|num| if num > &0 { 2_u32.pow(num - 1) } else { 0 })
        .sum();
    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let points = get_card_point_map(input);
    let len = points.len();
    let mut cards = vec![1; len];
    for (idx, point) in points.iter().enumerate() {
        let mut num = *point;
        let mut index = idx + 1;
        while index < len && num > 0 {
            cards[index] += cards[idx];
            index += 1;
            num -= 1;
        }
    }
    let num_cards: u32 = cards.iter().sum();
    Some(num_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_parse_cards() {
        let points = get_card_point_map(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(points, vec![4, 2, 2, 1, 0, 0]);
    }
}
