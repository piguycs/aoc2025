use std::{fmt::Write, ops::Range};

pub fn parse(input: &str) -> Vec<Range<usize>> {
    let input = input.replace("\n", "");
    input.split(",").map(|e| parse_range(e).unwrap()).collect()
}

/// Parses an input such as "11-22" and returns a range struct
fn parse_range(input: &str) -> Option<Range<usize>> {
    let mut splits = input.split("-");

    let start = splits.next().and_then(|e| str::parse::<usize>(e).ok())?;
    let end = splits.next().and_then(|e| str::parse::<usize>(e).ok())?;

    // exclusive as we are using an exclusive range struct
    let end_exclusive = end + 1;

    Some(start..end_exclusive)
}

pub fn solution_p1(ranges: Vec<Range<usize>>) -> usize {
    let mut buffer = String::with_capacity(32);

    let mut total = 0;

    for range in ranges {
        for i in range {
            let (a, b) = get_half_nums(i, &mut buffer);
            if a == b {
                total += i;
            }
        }
    }

    total
}

/// Something like 1234 would becoem (12, 34)
pub fn get_half_nums(num: usize, buffer: &mut String) -> (&str, &str) {
    buffer.clear();
    write!(buffer, "{num}").unwrap();
    buffer.split_at(buffer.len() / 2)
}

// fn get_digits(i: usize) -> u32 {
//     (i as f64).log10().floor() as u32 + 1
// }
