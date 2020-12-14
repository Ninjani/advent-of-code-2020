use crate::parse_usize;

/// cargo aoc bench results
/// Day13 - Part1/(default) time:   [346.44 ns 350.25 ns 354.89 ns]
/// Day13 - Part2/(default) time:   [1.3348 us 1.3395 us 1.3450 us]

#[aoc(day13, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    let index = memchr::memchr(b'\n', input).unwrap();
    let earliest = parse_usize(&input[..index]);
    let (closest_bus, time) = input[index + 1..]
        .split(|&b| b == b',')
        .filter(|b| b != b"x")
        .map(|b| {
            let b = parse_usize(b);
            let nearest = b * (earliest / b);
            if nearest >= earliest {
                (b, nearest)
            } else {
                (b, nearest + b)
            }
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    closest_bus * (time - earliest)
}

// Chinese remainder theorem
#[aoc(day13, part2)]
pub fn solve_part2(input: &[u8]) -> i64 {
    let mut modulii_residuals = Vec::with_capacity(10);
    let mut prod = 1;
    input[memchr::memchr(b'\n', input).unwrap() + 1..]
        .split(|&b| b == b',')
        .enumerate()
        .filter(|(_, b)| b != b"x")
        .for_each(|(i, b)| {
            let modulus = parse_usize(b) as i64;
            modulii_residuals.push((modulus, modulus - i as i64 + modulus));
            prod *= modulus;
        });
    #[inline(always)]
    fn egcd(a: i64, b: i64) -> (i64, i64) {
        if a == 0 {
            (0, 1)
        } else {
            let (x, y) = egcd(b % a, a);
            (y - (b / a) * x, x)
        }
    }
    modulii_residuals
        .into_iter()
        .fold(0, |sum, (modulus, residual)| {
            let p = prod / modulus;
            sum + residual * ((egcd(p, modulus).0 % modulus + modulus) % modulus) * p
        })
        % prod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = br#"939
7,13,x,x,59,x,31,19"#;
        assert_eq!(solve_part1(input), 295);
    }

    #[test]
    fn test_part2_1() {
        let input = br#"939
7,13,x,x,59,x,31,19"#;
        assert_eq!(solve_part2(input), 1068781);
    }
    #[test]
    fn test_part2_2() {
        let input = br#"939
        17,x,13,19"#;
        assert_eq!(solve_part2(input), 3417);
    }
    #[test]
    fn test_part2_3() {
        let input = br#"939
67,7,59,61"#;
        assert_eq!(solve_part2(input), 754018);
    }
    #[test]
    fn test_part2_4() {
        let input = br#"939
67,x,7,59,61"#;
        assert_eq!(solve_part2(input), 779210);
    }
    #[test]
    fn test_part2_5() {
        let input = br#"939
67,7,x,59,61"#;
        assert_eq!(solve_part2(input), 1261476);
    }
    #[test]
    fn test_part2_6() {
        let input = br#"939
1789,37,47,1889"#;
        assert_eq!(solve_part2(input), 1202161486);
    }
}
