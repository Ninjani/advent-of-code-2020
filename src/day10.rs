use crate::parse_usizes;

/// cargo aoc bench results
/// Day10 - Part1/(default) time:   [1.7502 us 1.9435 us 2.2603 us]
/// Day10 - Part2/(default) time:   [2.1823 us 2.2102 us 2.2493 us]

#[aoc(day10, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    let mut adaptors = parse_usizes(input);
    adaptors.sort_unstable();
    let (_, d1, d3) = adaptors
        .into_iter()
        .fold((0, 0, 0), |(prev, mut d1, mut d3), current| {
            match current - prev {
                1 => d1 += 1,
                3 => d3 += 1,
                _ => (),
            }
            (current, d1, d3)
        });
    d1 * (d3 + 1)
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    let mut adaptors = vec![0];
    adaptors.extend_from_slice(&parse_usizes(input));
    adaptors.sort_unstable();
    adaptors.push(adaptors[adaptors.len() - 1] + 3);
    let length = adaptors.len();
    let mut paths = [0; 100];
    paths[length - 1] = 1;
    for (i, adaptor) in adaptors.iter().enumerate().rev().skip(1) {
        paths[i] = (i + 1..i + 4)
            .map(|j| {
                if j < length && adaptors[j] - adaptor <= 3 {
                    paths[j]
                } else {
                    0
                }
            })
            .sum();
    }
    paths[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = br#"16
10
15
5
1
11
7
19
6
12
4"#;
        assert_eq!(solve_part1(&input), 35);
        let input = br#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
        assert_eq!(solve_part1(&input), 220);
    }

    #[test]
    fn test_part2() {
        let input = br#"16
10
15
5
1
11
7
19
6
12
4"#;
        assert_eq!(solve_part2(&input), 8);
        let input = br#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
        assert_eq!(solve_part2(&input), 19208);
    }
}
