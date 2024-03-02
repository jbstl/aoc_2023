use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;

advent_of_code::solution!(10);

type Point = (usize, usize);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
enum Tile {
    Vert,
    Hor,
    NE,
    NW,
    SW,
    SE,
    None,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::Vert),
            '-' => Ok(Tile::Hor),
            'L' => Ok(Tile::NE),
            'J' => Ok(Tile::NW),
            '7' => Ok(Tile::SW),
            'F' => Ok(Tile::SE),
            '.' => Ok(Tile::None),
            'S' => Ok(Tile::Start),
            _ => Err(anyhow!("Unable to parse value: {}", value)),
        }
    }
}

struct Node {
    tile: Tile,
    next: Option<Point>,
    prev: Option<Point>,
}

impl Node {
    fn new(tile: Tile) -> Self {
        Self {
            tile,
            next: None,
            prev: None,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Node>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    let tile = Tile::try_from(c).expect("Unable to parse input");
                    Node::new(tile)
                })
                .collect_vec()
        })
        .collect_vec()
}

fn get_next_loop_neighbor(
    input: &[Vec<Node>],
    row: usize,
    index: usize,
    direction: Option<Direction>,
) -> Option<(Point, Direction)> {
    let input_len = input.len();
    let row_len = input[0].len();
    if let Some(direction) = direction {
        // The next node is based on the value of the current node and the direction of the loop
        let current_tile = &input[row][index].tile;
        match current_tile {
            Tile::Vert => {
                if let Direction::Up = direction {
                    if row > 0 {
                        return Some(((row - 1, index), Direction::Up));
                    }
                } else if let Direction::Down = direction {
                    if row < input_len - 1 {
                        return Some(((row + 1, index), Direction::Down));
                    }
                }
            }
            Tile::Hor => {
                if let Direction::Left = direction {
                    if index > 0 {
                        return Some(((row, index - 1), Direction::Left));
                    }
                } else if let Direction::Right = direction {
                    if index < row_len - 1 {
                        return Some(((row, index + 1), Direction::Right));
                    }
                }
            }
            Tile::NE => {
                if let Direction::Down = direction {
                    if index < row_len - 1 {
                        return Some(((row, index + 1), Direction::Right));
                    }
                } else if let Direction::Left = direction {
                    if row > 0 {
                        return Some(((row - 1, index), Direction::Up));
                    }
                }
            }
            Tile::SE => {
                if let Direction::Up = direction {
                    if index < row_len - 1 {
                        return Some(((row, index + 1), Direction::Right));
                    }
                } else if let Direction::Left = direction {
                    if row < input_len - 1 {
                        return Some(((row + 1, index), Direction::Down));
                    }
                }
            }
            Tile::NW => {
                if let Direction::Down = direction {
                    if index > 0 {
                        return Some(((row, index - 1), Direction::Left));
                    }
                } else if let Direction::Right = direction {
                    if row > 0 {
                        return Some(((row - 1, index), Direction::Up));
                    }
                }
            }
            Tile::SW => {
                if let Direction::Up = direction {
                    if index > 0 {
                        return Some(((row, index - 1), Direction::Left));
                    }
                } else if let Direction::Right = direction {
                    if row < input_len - 1 {
                        return Some(((row + 1, index), Direction::Down));
                    }
                }
            }
            _ => {}
        };
    } else {
        // If no direction is provided, check all neighbors counterclockwise and return the first one
        // Check above
        if row > 0 {
            let neighbor = &input[row - 1][index];
            if let Tile::Start | Tile::Vert | Tile::SW | Tile::SE = neighbor.tile {
                return Some(((row - 1, index), Direction::Up));
            };
        }
        // Check right
        if index < row_len - 1 {
            let neighbor = &input[row][index + 1];
            if let Tile::Start | Tile::Hor | Tile::NW | Tile::SW = neighbor.tile {
                return Some(((row, index + 1), Direction::Right));
            };
        }
        // Check below
        if row < input_len - 1 {
            let neighbor = &input[row + 1][index];
            if let Tile::Start | Tile::Vert | Tile::NW | Tile::NE = neighbor.tile {
                return Some(((row + 1, index), Direction::Down));
            };
        }
        // Check left
        if index > 0 {
            let neighbor = &input[row][index - 1];
            if let Tile::Start | Tile::Hor | Tile::NE | Tile::SE = neighbor.tile {
                return Some(((row, index - 1), Direction::Left));
            };
        }
    }
    None
}

fn get_starting_index(input: &[Vec<Node>]) -> Option<Point> {
    let mut start_idx = None;
    'outer: for (row, line) in input.iter().enumerate() {
        for (index, node) in line.iter().enumerate() {
            if node.tile == Tile::Start {
                start_idx = Some((row, index));
                break 'outer;
            }
        }
    }
    start_idx
}

fn mark_loop(input: &mut [Vec<Node>]) -> usize {
    let start_idx = get_starting_index(input).expect("Unable to find starting tile");
    let mut last_idx = start_idx;
    let mut dir = None;
    let mut steps = 0;
    while let Some((node_idx, direction)) =
        get_next_loop_neighbor(input, last_idx.0, last_idx.1, dir)
    {
        steps += 1;
        // Set the previous node
        input[node_idx.0][node_idx.1].prev = Some((last_idx.0, last_idx.1));
        // Set the next node
        input[last_idx.0][last_idx.1].next = Some((node_idx.0, node_idx.1));
        dir = Some(direction);
        if node_idx == start_idx {
            break;
        }
        last_idx = node_idx;
    }
    steps
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse_input(input);
    let points = mark_loop(&mut input);
    Some(points / 2)
}

// To find the number of points that exist in a polygon, we can use Pick's Theorem to relate the
// area of a polygon to the number of interior points and vertices. To find the area of a polygon,
// use the shoelace formula.
// https://en.wikipedia.org/wiki/Pick%27s_theorem
// https://en.wikipedia.org/wiki/Shoelace_formula
pub fn part_two(input: &str) -> Option<usize> {
    let mut input = parse_input(input);
    let points = mark_loop(&mut input);
    let start_idx = get_starting_index(&input).expect("Unable to find starting tile");

    // shoelace formula
    let mut current_node = start_idx;
    let mut sum1 = 0;
    let mut sum2 = 0;
    for _ in 0..points {
        let node = &input[current_node.0][current_node.1];
        let next_node = node.next.unwrap();
        sum1 += current_node.0 * next_node.1;
        sum2 += current_node.1 * next_node.0;
        current_node = next_node;
    }
    let area = ((sum1 as i32 - sum2 as i32) / 2).abs();

    // Pick's theorem
    let interior_points = area as usize - (points / 2) + 1;

    Some(interior_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }
}
