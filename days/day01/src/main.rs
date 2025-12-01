use day01::*;

fn main() {
    let input = include_str!("../../../inputs/day01.txt");
    let actions = Action::parse(input);
    let sol = solution_part1(&actions, 50);
    println!("Solution: {}", sol);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn example() {
        let actions = Action::parse(INPUT);
        let sol = solution_part1(&actions, 50);
        assert_eq!(sol, 3);
    }
}
