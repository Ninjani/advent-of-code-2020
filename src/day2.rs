use itertools::Itertools;

pub struct Policy {
    n1: usize,
    n2: usize,
    letter: u8,
}

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<(Policy, String)> {
    input
        .split('\n')
        .map(|line| {
            let (policy, password) = line.trim().split(": ").collect_tuple().unwrap();
            let (numbers, letter) = policy.split(' ').collect_tuple().unwrap();
            let (n1, n2) = numbers
                .split('-')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            (
                Policy {
                    n1,
                    n2,
                    letter: letter.as_bytes()[0],
                },
                password.to_owned(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| {
            let count = password.bytes().filter(|c| c == &policy.letter).count();
            count >= policy.n1 && count <= policy.n2
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| {
            let chars = password.as_bytes();
            match (chars.get(policy.n1 - 1), chars.get(policy.n2 - 1)) {
                (Some(a), Some(b)) => {
                    (a == &policy.letter || b == &policy.letter)
                        && !(a == &policy.letter && b == &policy.letter)
                }
                (Some(a), None) => a == &policy.letter,
                (None, Some(b)) => b == &policy.letter,
                (None, None) => false,
            }
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
