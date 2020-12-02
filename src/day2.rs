use parse_display::{Display, FromStr};
use rayon::prelude::*;

/// cargo aoc bench results
/// Generator Day2/(default) time:   [977.92 us 1.0102 ms 1.0453 ms]
/// Day2 - Part1/(default)  time:   [8.0064 us 8.2369 us 8.5390 us]
/// Day2 - Part2/(default)  time:   [2.0522 us 2.0620 us 2.0735 us]

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{n1}-{n2} {letter}: {password}")]
pub struct Policy {
    n1: usize,
    n2: usize,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<Policy> {
    input
        .par_lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Policy]) -> usize {
    input
        .iter()
        .filter(|policy| {
            let count = policy
                .password
                .bytes()
                .filter(|c| c == &(policy.letter as u8))
                .count();
            count >= policy.n1 && count <= policy.n2
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Policy]) -> usize {
    input
        .iter()
        .filter(|policy| {
            let chars = policy.password.as_bytes();
            (chars.get(policy.n1 - 1) == Some(&(policy.letter as u8)))
                != (chars.get(policy.n2 - 1) == Some(&(policy.letter as u8)))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = generate("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = generate("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(solve_part2(&input), 1);
    }
}
