use std::ops::Range;

pub fn parse(input: &str) -> Vec<Range<usize>> {
    input.split(",").filter_map(parse_range).collect()
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
    let mut total = 0;

    for range in ranges {
        let mut local_total = 0;
        let mut count = 0;

        println!("{range:<20?}");

        for i in range {
            let (a, b) = get_half_nums(i);
            if a == b {
                println!("susy: {i} a={a} b={b}");
                local_total += i;
                count += 1;
            }
        }

        println!("{local_total:<10} {count}\n\n");

        total += local_total;
    }

    total
}

/// Something like 1234 would becoem (12, 34)
/// Something like 1001 should return (1, 1) as zeros need to be culled
pub fn get_half_nums(num: usize) -> (usize, usize) {
    const BASE: usize = 10;

    let digits = get_digits(num);

    let midder = BASE.pow(digits / 2);

    let left = num / midder;
    let right = num % midder;

    (left, right)
}

fn get_digits(i: usize) -> u32 {
    (i as f64).log10().floor() as u32 + 1
}
