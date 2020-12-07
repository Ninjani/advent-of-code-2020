use itertools::Itertools;

/// Custom parsing + &[u8] instead of &str

/// cargo aoc bench results
/// Generator Day2/(default) time:   [179.93 us 181.62 us 183.53 us]
/// Day2 - Part1/(default)  time:   [8.2080 us 8.4422 us 8.7164 us]
/// Day2 - Part2/(default)  time:   [2.5433 us 2.5843 us 2.6369 us]

pub struct Policy {
    n1: usize,
    n2: usize,
    letter: u8,
    password: Vec<u8>,
}

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<Policy> {
    input
        .as_bytes()
        .split(|b| b == &b'\n')
        .map(|line| {
            let (n1_n2, letter, _, password) = line
                .split(|b| b == &b' ' || b == &b':')
                .collect_tuple()
                .unwrap();
            let (n1, n2) = n1_n2
                .split(|b| b == &b'-')
                .map(|n| std::str::from_utf8(n).unwrap().parse().unwrap())
                .collect_tuple()
                .unwrap();
            Policy {
                n1,
                n2,
                letter: letter[0],
                password: password.to_vec(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Policy]) -> usize {
    input
        .iter()
        .filter(|policy| {
            let count: usize = bytecount::count(&policy.password, policy.letter);
            count >= policy.n1 && count <= policy.n2
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Policy]) -> usize {
    input
        .iter()
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
        let input = generate("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = generate("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(solve_part2(&input), 1);
    }
}
