advent_of_code::solution!(11);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pixel {
    Empty,
    Galaxy,
}

type Image = Vec<Vec<Pixel>>;
type Galaxies = Vec<(usize, usize)>;

fn parse_input(input: &str) -> Image {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let pixel = match c {
                '.' => Pixel::Empty,
                '#' => Pixel::Galaxy,
                _ => unreachable!(),
            };
            row.push(pixel);
        }
        result.push(row);
    }
    result
}

fn get_empty_rows(image: &Image) -> Vec<usize> {
    let mut empty_rows = Vec::new();
    for (index, row) in image.iter().enumerate() {
        let galaxy_count = row.iter().filter(|&&pixel| pixel == Pixel::Galaxy).count();
        if galaxy_count == 0 {
            empty_rows.push(index)
        }
    }
    empty_rows
}

fn get_empty_columns(image: &Image) -> Vec<usize> {
    let mut empty_columns = Vec::new();
    let column_count = image[0].len();
    let mut empty_column_map = vec![true; column_count];
    for row in image {
        for (index, &pixel) in row.iter().enumerate() {
            if empty_column_map[index] && pixel == Pixel::Galaxy {
                empty_column_map[index] = false;
            }
        }
    }

    for (index, &empty) in empty_column_map.iter().enumerate() {
        if empty {
            empty_columns.push(index);
        }
    }

    empty_columns
}

fn get_galaxies(image: &Image) -> Galaxies {
    let mut galaxies = Vec::new();
    for (row_index, row) in image.iter().enumerate() {
        for (col_index, &pixel) in row.iter().enumerate() {
            if pixel == Pixel::Galaxy {
                galaxies.push((row_index, col_index));
            }
        }
    }
    galaxies
}

fn expand_galaxies(
    galaxies: Galaxies,
    empty_rows: &[usize],
    empty_columms: &[usize],
    expansion_amount: usize,
) -> Galaxies {
    let mut new_galaxies = Vec::new();
    for (x, y) in galaxies {
        let empty_rows_before = empty_rows.iter().filter(|&&idx| idx < x).count();
        let empty_columns_before = empty_columms.iter().filter(|&&idx| idx < y).count();
        let new_x = x + (empty_rows_before * expansion_amount);
        let new_y = y + (empty_columns_before * expansion_amount);
        new_galaxies.push((new_x, new_y));
    }
    new_galaxies
}

fn get_galaxy_distances(input: &str, expansion_amount: usize) -> usize {
    let image = parse_input(input);
    let galaxies = get_galaxies(&image);
    let empty_rows = get_empty_rows(&image);
    let empty_columms = get_empty_columns(&image);
    let galaxies = expand_galaxies(galaxies, &empty_rows, &empty_columms, expansion_amount);
    let galaxy_count = galaxies.len();
    let mut sum = 0;
    for (index, (x, y)) in galaxies.iter().enumerate() {
        if index < galaxy_count - 1 {
            for (next_x, next_y) in &galaxies[(index + 1)..galaxy_count] {
                sum += (*x as isize - *next_x as isize).unsigned_abs();
                sum += (*y as isize - *next_y as isize).unsigned_abs();
            }
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(get_galaxy_distances(input, 1))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(get_galaxy_distances(input, 1000000 - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(get_galaxy_distances(&input, 10 - 1), 1030);
        assert_eq!(get_galaxy_distances(&input, 100 - 1), 8410);
    }

    #[test]
    fn test_empty_rows_and_columns() {
        let img = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let empty_columms = get_empty_columns(&img);
        assert_eq!(empty_columms, vec![2, 5, 8]);
        let empty_rows = get_empty_rows(&img);
        assert_eq!(empty_rows, vec![3, 7]);
    }
}
