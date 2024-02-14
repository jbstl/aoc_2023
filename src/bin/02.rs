advent_of_code::solution!(2);

use std::u32;

use anyhow::{Ok, Result};
use pest::Parser;
use pest_derive::Parser;

/// Subset of red, green, and blue cubes pulled from bag.
#[derive(Default, Debug, PartialEq, Eq)]
struct SubSet(u32, u32, u32);

#[derive(Default, Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    subsets: Vec<SubSet>,
}

impl Game {
    fn add_subset(&mut self, subset: SubSet) {
        self.subsets.push(subset);
    }

    fn id(&mut self, id: u32) {
        self.id = id;
    }

    fn subsets(&self) -> &[SubSet] {
        &self.subsets
    }
}

#[derive(Parser)]
#[grammar = "day2.pest"]
struct GameParser;

fn parse_game(input: &str) -> Result<Game> {
    let game_parse = GameParser::parse(Rule::game, input)
        .unwrap()
        .next()
        .unwrap();

    let mut game = Game::default();
    for item in game_parse.into_inner() {
        match item.as_rule() {
            Rule::game_num => {
                let num = item.into_inner().next().unwrap();
                let num = num.as_str().parse::<u32>().unwrap();
                game.id(num);
            }
            Rule::subset => {
                let mut subset = SubSet::default();
                let subsets = item.into_inner();
                for cube in subsets {
                    let cube = cube.into_inner().next().unwrap();
                    match cube.as_rule() {
                        Rule::red_cube => {
                            let num = cube.into_inner().next().unwrap();
                            let num = num.as_str().parse::<u32>().unwrap();
                            subset.0 = num;
                        }
                        Rule::green_cube => {
                            let num = cube.into_inner().next().unwrap();
                            let num = num.as_str().parse::<u32>().unwrap();
                            subset.1 = num;
                        }
                        Rule::blue_cube => {
                            let num = cube.into_inner().next().unwrap();
                            let num = num.as_str().parse::<u32>().unwrap();
                            subset.2 = num;
                        }
                        _ => unreachable!(),
                    }
                }
                game.add_subset(subset);
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    Ok(game)
}

fn parse_games(input: &str) -> Result<Vec<Game>> {
    let mut games = Vec::new();
    for line in input.lines() {
        games.push(parse_game(line)?);
    }
    Ok(games)
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_games(input).unwrap();
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let total = games.iter().fold(0, |mut sum, game| {
        let id = game.id;
        let mut game_possible = true;
        for subset in game.subsets() {
            if !((subset.0 <= max_red) && (subset.1 <= max_green) && (subset.2 <= max_blue)) {
                game_possible = false;
                break;
            }
        }
        if game_possible {
            sum += id
        }
        sum
    });
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_games(input).unwrap();
    let total = games.iter().fold(0, |mut sum, game| {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let mut game_total = 1;
        for subset in game.subsets() {
            let red = subset.0;
            let green = subset.1;
            let blue = subset.2;
            if red > max_red {
                max_red = red;
            }
            if green > max_green {
                max_green = green;
            }
            if blue > max_blue {
                max_blue = blue;
            }
        }
        if max_red != 0 {
            game_total *= max_red;
        }
        if max_blue != 0 {
            game_total *= max_blue;
        }
        if max_green != 0 {
            game_total *= max_green;
        }
        if game_total > 1 {
            sum += game_total;
        }
        sum
    });
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn test_parse_game() {
        let game_str = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let parse_result = parse_game(game_str).unwrap();
        let mut game = Game::default();
        game.id(4);
        game.add_subset(SubSet(3, 1, 6));
        game.add_subset(SubSet(6, 3, 0));
        game.add_subset(SubSet(14, 3, 15));
        assert_eq!(parse_result, game);
    }
}
