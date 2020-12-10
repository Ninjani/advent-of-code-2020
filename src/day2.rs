use crate::parse_usize;
use itertools::Itertools;

/// cargo aoc bench results
/// Day2 - Part1/(default)  time:   [67.795 us 69.432 us 71.462 us]
/// Day2 - Part2/(default)  time:   [58.868 us 59.981 us 61.275 us]

pub struct Policy<'a> {
    n1: usize,
    n2: usize,
    letter: u8,
    password: &'a [u8],
}

#[inline(always)]
pub fn generate(input: &[u8]) -> impl Iterator<Item = Policy> {
    input.split(|b| b == &b'\n').map(|line| {
        let (n1_n2, letter, _, password) = line
            .split(|b| b == &b' ' || b == &b':')
            .collect_tuple()
            .unwrap();
        let (n1, n2) = n1_n2
            .split(|b| b == &b'-')
            .map(|n| parse_usize(n))
            .collect_tuple()
            .unwrap();
        Policy {
            n1,
            n2,
            letter: letter[0],
            password,
        }
    })
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    generate(input)
        .filter(|policy| {
            let count: usize = bytecount::count(&policy.password, policy.letter);
            count >= policy.n1 && count <= policy.n2
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    generate(input)
        .filter(|policy| {
            (policy.password[policy.n1 - 1] == policy.letter)
                != (policy.password[policy.n2 - 1] == policy.letter)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = b"1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = b"1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(solve_part2(&input), 1);
    }
}
