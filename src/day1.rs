use crate::parse_usizes;
use std::cmp::Ordering;

/// cargo aoc bench results:
/// Day1 - Part1/(default)  time:   [3.7296 us 3.8086 us 3.9079 us]
/// Day1 - Part2/(default)  time:   [3.5171 us 3.5446 us 3.5789 us]

#[inline(always)]
pub fn two_sum(sorted_array: &[usize], target: usize) -> Option<(usize, usize)> {
    let length = sorted_array.len();
    let (mut start, mut end) = (0, length - 1);
    while start < end {
        let sum = sorted_array[start] + sorted_array[end];
        match sum.cmp(&target) {
            Ordering::Equal => return Some((sorted_array[start], sorted_array[end])),
            Ordering::Greater => end -= 1,
            Ordering::Less => start += 1,
        }
    }
    None
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    let mut input = parse_usizes(input);
    input.sort_unstable();
    let (i, j) = two_sum(&input, 2020).unwrap();
    i * j
}

#[inline(always)]
fn three_sum(sorted_array: &[usize], target: usize) -> Option<(usize, usize, usize)> {
    let length = sorted_array.len();
    for i in 0..=(length - 2) {
        let a = sorted_array[i];
        let (mut start, mut end) = (i + 1, length - 1);
        while start < end {
            let sum = a + sorted_array[start] + sorted_array[end];
            match sum.cmp(&target) {
                Ordering::Equal => {
                    return Some((sorted_array[i], sorted_array[start], sorted_array[end]))
                }
                Ordering::Greater => end -= 1,
                Ordering::Less => start += 1,
            }
        }
    }
    None
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    let mut input = parse_usizes(input);
    input.sort_unstable();
    let (i, j, k) = three_sum(&input, 2020).unwrap();
    i * j * k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = b"1721\n979\n366\n299\n675\n1456";
        assert_eq!(solve_part1(&input), 514579);
    }

    #[test]
    fn test_part2() {
        let input = b"1721\n979\n366\n299\n675\n1456";
        assert_eq!(solve_part2(&input), 241861950);
    }

    #[test]
    fn test_half() {
        let input = b"1010\n2020\n0\n299\n670\n14";
        assert_eq!(solve_part1(&input), 0);
    }
}
