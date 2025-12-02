use day02::*;

fn main() {
    let input = include_str!("../../../inputs/day02.txt");
    let ranges = parse(input);

    let sol1 = solution_p1(ranges);
    println!("Solution p1: {sol1}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn solution01() {
        let ranges = parse(INPUT);
        let sol = solution_p1(ranges);
        assert_eq!(sol, 1227775554);
    }

    #[test]
    // Just making sure I still remember some basic maths stuff
    fn maths_ops_test() {
        let assert_digits = |i: i32, expected: u32| {
            let i = i as f64;
            let digits = i.log10().floor() as u32 + 1;
            assert_eq!(digits, expected, "num digits for {i} should be {expected}");
        };

        for i in 0..=9 {
            assert_digits(i, 1);
        }

        for i in 10..=99 {
            assert_digits(i, 2);
        }

        for i in 100..=999 {
            assert_digits(i, 3);
        }

        for i in 1000..=9999 {
            assert_digits(i, 4);
        }
    }

    #[test]
    // building on top of the previous test to test more maths
    fn more_maths_ops_test() {
        let mut buffer = String::with_capacity(1024);

        let (a, b) = get_half_nums(1234, &mut buffer);
        assert_eq!(a, "12");
        assert_eq!(b, "34");
        let (a, b) = get_half_nums(12345678, &mut buffer);
        assert_eq!(a, "1234");
        assert_eq!(b, "5678");
    }

    #[test]
    // realised this would be a failing case for p1
    fn failing_case_p1() {
        let mut buffer = String::with_capacity(1024);
        let (a, b) = get_half_nums(1001, &mut buffer);
        assert_eq!(a, "10");
        assert_eq!(b, "01");
    }
}
