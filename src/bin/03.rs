advent_of_code::solution!(3);

use std::{char, collections::HashMap, ops::RangeInclusive};

use anyhow::{Ok, Result};
use pest::Parser;
use pest_derive::Parser;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum SchematicValue {
    Symbol(char),
    Number(usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SchematicItem {
    kind: SchematicValue,
    row: usize,
    start_pos: usize,
    end_pos: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Schematic {
    right_index: usize,
    items: Vec<Vec<SchematicItem>>,
}

impl Schematic {
    fn new(right_index: usize) -> Self {
        let items = Vec::new();
        Self { right_index, items }
    }

    fn get_item_kind_in_row_range(
        &self,
        kind: &SchematicValue,
        row_num: usize,
        range: RangeInclusive<usize>,
    ) -> Option<Vec<&SchematicItem>> {
        let row = &self.items[row_num];
        let items: Vec<&SchematicItem> = row
            .iter()
            .filter(|&item| match kind {
                SchematicValue::Number(_) => matches!(item.kind, SchematicValue::Number(_)),
                SchematicValue::Symbol(_) => matches!(item.kind, SchematicValue::Symbol(_)),
            })
            .filter(|&item| range.contains(&item.start_pos) | range.contains(&item.end_pos))
            .collect();
        if items.is_empty() {
            None
        } else {
            Some(items)
        }
    }

    fn get_item_neighbors(&self, row: usize, index: usize) -> Option<Vec<&SchematicItem>> {
        let mut vec = Vec::new();
        let item = &self.items[row][index];

        let left_limit = if item.start_pos == 0 {
            item.start_pos
        } else {
            item.start_pos - 1
        };
        let right_limit = if item.end_pos == self.right_index {
            item.end_pos
        } else {
            item.end_pos + 1
        };
        let neighbor_kind = match item.kind {
            SchematicValue::Symbol(_) => SchematicValue::Number(0),
            SchematicValue::Number(_) => SchematicValue::Symbol('a'),
        };

        // get neighbors from row above
        if item.row > 0 {
            let neighbors = self.get_item_kind_in_row_range(
                &neighbor_kind,
                item.row - 1,
                left_limit..=right_limit,
            );
            if let Some(mut neighbors) = neighbors {
                vec.append(&mut neighbors);
            }
        }
        // get neighbors from the same row
        let neighbors =
            self.get_item_kind_in_row_range(&neighbor_kind, item.row, left_limit..=right_limit);
        if let Some(mut neighbors) = neighbors {
            vec.append(&mut neighbors);
        }
        // get neighbors from row below
        if item.row < self.items.len() - 1 {
            let neighbors = self.get_item_kind_in_row_range(
                &neighbor_kind,
                item.row + 1,
                left_limit..=right_limit,
            );
            if let Some(mut neighbors) = neighbors {
                vec.append(&mut neighbors);
            }
        }
        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn get_schematic_neighbors(
        &self,
        kind: SchematicValue,
    ) -> HashMap<&SchematicItem, Vec<&SchematicItem>> {
        let mut map = HashMap::new();
        for (row_num, row) in self.items.iter().enumerate() {
            for (item_num, item) in row.iter().enumerate() {
                match item.kind {
                    SchematicValue::Symbol(_) => {
                        if matches!(kind, SchematicValue::Symbol(_)) {
                            if let Some(neighbors) = self.get_item_neighbors(row_num, item_num) {
                                map.insert(item, neighbors);
                            }
                        }
                    }
                    SchematicValue::Number(_) => {
                        if matches!(kind, SchematicValue::Number(_)) {
                            if let Some(neighbors) = self.get_item_neighbors(row_num, item_num) {
                                map.insert(item, neighbors);
                            }
                        }
                    }
                }
            }
        }
        map
    }
}

#[derive(Parser)]
#[grammar = "parsers/day3.pest"]
struct SchematicParser;

fn parse_schematic(input: &str) -> Result<Schematic> {
    let r_index = input.lines().next().unwrap().chars().count() - 1;
    let line_parse = SchematicParser::parse(Rule::schematic, input)
        .unwrap()
        .next()
        .unwrap();
    let mut schematic = Schematic::new(r_index);
    for item in line_parse.into_inner() {
        if let Rule::line = item.as_rule() {
            let mut line = Vec::new();
            let row = schematic.items.len();
            let values = item.into_inner();
            for value in values {
                match value.as_rule() {
                    Rule::num => {
                        // line col returns the byte location, so it starts at 1
                        let (_, start_pos) = value.line_col();
                        // let num = value.into_inner().next().unwrap();
                        let num = value.as_str();
                        let num_len = num.len();
                        let num = num.parse::<usize>().unwrap();
                        let start_pos = start_pos - 1;
                        let end_pos = start_pos + num_len - 1;
                        let val = SchematicItem {
                            row,
                            kind: SchematicValue::Number(num),
                            start_pos,
                            end_pos,
                        };
                        line.push(val);
                    }
                    Rule::symbol => {
                        let (_, pos) = value.line_col();
                        let c = value
                            .as_str()
                            .chars()
                            .next()
                            .expect("Unable to get symbol char");
                        let val = SchematicItem {
                            row,
                            kind: SchematicValue::Symbol(c),
                            start_pos: pos - 1,
                            end_pos: pos - 1,
                        };
                        line.push(val);
                    }
                    _ => {}
                }
            }
            schematic.items.push(line);
        }
    }
    Ok(schematic)
}

pub fn part_one(input: &str) -> Option<usize> {
    let schematic = parse_schematic(input).unwrap();
    let neighbor = schematic.get_schematic_neighbors(SchematicValue::Number(0));
    let total = neighbor.keys().fold(0, |mut acc, item| {
        if let SchematicValue::Number(num) = item.kind {
            acc += num;
        }
        acc
    });
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let schematic = parse_schematic(input).unwrap();
    let neighbors = schematic.get_schematic_neighbors(SchematicValue::Symbol('*'));
    let total = neighbors
        .iter()
        .filter(|(item, nearby)| {
            matches!(item.kind, SchematicValue::Symbol('*')) && nearby.len() == 2
        })
        .map(|(_, nearby)| {
            let mut mult = 1;
            for item in nearby {
                match item.kind {
                    SchematicValue::Number(num) => mult *= num,
                    _ => unreachable!(),
                }
            }
            mult
        })
        .sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }

    #[test]
    fn test_parse_schematic() {
        let line = "467..114..\n*........*";
        let num1 = SchematicItem {
            kind: SchematicValue::Number(467),
            row: 0,
            start_pos: 0,
            end_pos: 2,
        };
        let num2 = SchematicItem {
            kind: SchematicValue::Number(114),
            row: 0,
            start_pos: 5,
            end_pos: 7,
        };
        let sym1 = SchematicItem {
            kind: SchematicValue::Symbol('*'),
            row: 1,
            start_pos: 0,
            end_pos: 0,
        };
        let sym2 = SchematicItem {
            kind: SchematicValue::Symbol('*'),
            row: 1,
            start_pos: 9,
            end_pos: 9,
        };
        let schematic = Schematic {
            right_index: 9,
            items: vec![vec![num1, num2], vec![sym1, sym2]],
        };
        assert_eq!(parse_schematic(line).unwrap(), schematic);
    }
}
