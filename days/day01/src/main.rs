use day01::*;

fn main() {
    let input = include_str!("../../../inputs/day01.txt");
    let actions = Action::parse(input);
    let sol1 = solution_part1(&actions, 50);
    println!("Solution (part1): {}", sol1);
    let sol2 = solution_part2(&actions, 50);
    println!("Solution (part2): {}", sol2);
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
    fn example_part1() {
        let actions = Action::parse(INPUT);
        let sol = solution_part1(&actions, 50);
        assert_eq!(sol, 3);
    }

    #[test]
    fn example_part2() {
        let actions = Action::parse(INPUT);
        let sol = solution_part2(&actions, 50);
        assert_eq!(sol, 6);
    }
}
