advent_of_code::solution!(1);

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

enum BufMatch {
    Full,
    Partial,
    None,
}

// Match the buffer window and determine whether it is a full, partial, or non match for entry in
// NUMS.
fn buf_match(buf: &str) -> BufMatch {
    for num in NUMS {
        if num == buf {
            return BufMatch::Full;
        }
        if num.starts_with(buf) {
            return BufMatch::Partial;
        }
    }
    BufMatch::None
}

// Shrink buffer by removing one character at a time from the left and checking for a partial
// match.
fn shrink_buffer(input: &mut String) {
    let len = input.len();
    let mut slice: Option<usize> = None;
    'outer: for num in 0..len {
        let buf_slice = &input[num..len];
        if let BufMatch::Partial = buf_match(buf_slice) {
            slice = Some(num);
            break 'outer;
        }
    }
    if let Some(index) = slice {
        *input = input.chars().skip(index).collect();
    } else {
        input.clear()
    }
}

// Convert the string representation of a number to the digit.
fn str_num_to_digit(input: &str) -> Option<u32> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .trim()
        .lines()
        .map(|line| {
            let (first, last) =
                line.chars()
                    .fold((None, None), |(mut first_num, mut last_num), c| {
                        if c.is_ascii_digit() {
                            let val = c.to_digit(10);
                            if first_num.is_none() {
                                first_num = val;
                            }
                            last_num = val;
                        };
                        (first_num, last_num)
                    });
            (first.unwrap() * 10) + last.unwrap()
        })
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total = input
        .trim()
        .lines()
        .map(|line| {
            let (first, last, _buf) = line.chars().fold(
                (None, None, String::new()),
                |(mut first_num, mut last_num, mut buf), c| {
                    if c.is_ascii_digit() {
                        buf.clear();
                        let val = c.to_digit(10);
                        if first_num.is_none() {
                            first_num = val;
                        }
                        last_num = val;
                    } else {
                        buf.push(c);
                        match buf_match(&buf) {
                            BufMatch::Full => {
                                let val = str_num_to_digit(&buf);
                                if first_num.is_none() {
                                    first_num = val;
                                }
                                last_num = val;
                                shrink_buffer(&mut buf);
                            }
                            BufMatch::Partial => {}
                            BufMatch::None => shrink_buffer(&mut buf),
                        }
                    };
                    (first_num, last_num, buf)
                },
            );
            (first.unwrap() * 10) + last.unwrap()
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(202));
    }
}
