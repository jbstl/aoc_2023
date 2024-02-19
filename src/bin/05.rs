use std::{ops::RangeInclusive, u32};
advent_of_code::solution!(5);

// For each section in the input, create a SrcMap of the section and it's children
fn parse_input(input: &str) -> (SrcMap, Vec<u32>) {
    let mut sections_str = input.split("\n\n");
    let seeds = sections_str.next().expect("Unable to parse seeds");

    let seeds: Vec<u32> = seeds
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let mut sections = Vec::new();

    for src_map in &mut sections_str {
        let mut maps: Vec<MapEntry> = src_map
            .lines()
            .skip(1) // skip title line
            .map(|line| {
                let mut nums = line.split_whitespace();
                let dest_start = nums
                    .next()
                    .expect("Unable to retrieve destination map start value")
                    .parse::<u32>()
                    .expect("Unable to parse destination map start value");
                let src_start = nums
                    .next()
                    .expect("Unable to retrieve source map start value")
                    .parse::<u32>()
                    .expect("Unable to parse source map start value");
                let len = nums
                    .next()
                    .expect("Unable to retrieve source map length")
                    .parse::<u32>()
                    .expect("Unable to parse source map length");
                MapEntry::new(src_start..=(src_start + len - 1), dest_start)
            })
            .collect();

        maps.sort_by(|a, b| a.range.start().cmp(b.range.start()));

        // fill out map for every value from 0 to U32MAX
        let mut max = 0;
        let last_index = maps.len() - 1;
        let mut map_entries = Vec::new();
        for (index, map) in maps.iter().enumerate() {
            let range_start = *map.range.start();
            let range_end = *map.range.end();
            if max < range_start {
                // add a new entry for the range before this one
                let new_entry = MapEntry::new(max..=(range_start - 1), max);
                map_entries.push(new_entry);
            }
            if range_end < u32::MAX {
                max = range_end + 1;
            }
            // Add a range for up to u32 max for the last iteration
            if index == last_index && max < u32::MAX {
                let new_entry = MapEntry::new(max..=u32::MAX, max);
                map_entries.push(new_entry);
            }
        }
        maps.append(&mut map_entries);
        sections.push(maps);
    }

    // Build the main map from the vector of vectors of map entries.
    let mut latest_section: Option<SrcMap> = None;
    while let Some(section) = sections.pop() {
        if latest_section.is_none() {
            latest_section = Some(SrcMap::new(section, None));
        } else {
            latest_section = Some(SrcMap::new(
                section,
                Some(Box::new(latest_section.unwrap())),
            ));
        }
    }

    (latest_section.unwrap(), seeds)
}

fn ranges_overlap(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    range1.start() <= range2.end() && range2.start() <= range1.end()
}

fn get_range_overlap(
    range1: &RangeInclusive<u32>,
    range2: &RangeInclusive<u32>,
) -> Option<RangeInclusive<u32>> {
    if ranges_overlap(range1, range2) {
        let overlap_start: u32 = *std::cmp::max(range1.start(), range2.start());
        let overlap_end: u32 = *std::cmp::min(range1.end(), range2.end());
        Some(overlap_start..=overlap_end)
    } else {
        None
    }
}

// A nested represtentation of the map transformations required by the almanac.
struct SrcMap {
    entries: Vec<MapEntry>,
    child_map: Option<Box<SrcMap>>,
}

impl SrcMap {
    fn new(entries: Vec<MapEntry>, child_map: Option<Box<SrcMap>>) -> Self {
        Self { entries, child_map }
    }

    fn get_dest_val(&self, src: &u32) -> u32 {
        let mut dest_val = None;
        for entry in &self.entries {
            if let Some(val) = entry.get_dest_val(src) {
                dest_val = Some(val);
                break;
            }
        }
        let dest_val = dest_val.unwrap();
        if let Some(child) = &self.child_map {
            child.get_dest_val(&dest_val)
        } else {
            dest_val
        }
    }

    fn get_dest_ranges(&self, range: RangeInclusive<u32>) -> Vec<RangeInclusive<u32>> {
        let mut dest_ranges = Vec::new();
        for entry in &self.entries {
            if let Some(range) = entry.get_dest_range(&range) {
                dest_ranges.push(range);
            }
        }

        if let Some(child_map) = &self.child_map {
            let mut child_ranges = Vec::new();
            for dest_range in dest_ranges {
                child_ranges.push(child_map.get_dest_ranges(dest_range));
            }
            child_ranges.into_iter().flatten().collect()
        } else {
            dest_ranges
        }
    }
}

struct MapEntry {
    range: RangeInclusive<u32>,
    dest_start: u32,
}

impl MapEntry {
    fn new(range: RangeInclusive<u32>, dest_start: u32) -> Self {
        Self { range, dest_start }
    }

    fn get_dest_val(&self, src: &u32) -> Option<u32> {
        if self.range.contains(src) {
            Some(self.dest_start + (src - self.range.start()))
        } else {
            None
        }
    }

    fn get_dest_range(&self, src: &RangeInclusive<u32>) -> Option<RangeInclusive<u32>> {
        if let Some(overlap) = get_range_overlap(&self.range, src) {
            let dest_range_start = self.dest_start + overlap.start() - self.range.start();
            let dest_range_end = dest_range_start + overlap.end() - overlap.start();
            Some(dest_range_start..=dest_range_end)
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, seeds) = parse_input(input);
    let first_plant = seeds
        .iter()
        .map(|seed| map.get_dest_val(seed))
        .min()
        .unwrap();
    Some(first_plant)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, seeds) = parse_input(input);
    let seed_ranges = seeds
        .chunks_exact(2)
        .map(|chunk| {
            let first_val = chunk[0];
            let range_size = chunk[1];
            let last_val = first_val + range_size - 1;
            first_val..=last_val
        })
        .collect::<Vec<RangeInclusive<u32>>>();

    let min_plant = seed_ranges
        .into_iter()
        .flat_map(|seed_range| map.get_dest_ranges(seed_range))
        .min_by_key(|range| *range.start())
        .unwrap();

    Some(*min_plant.start())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
