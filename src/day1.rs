use std::cmp::Ordering;

/// cargo aoc bench results:
/// Generator Day1/(default)  time:   [6.7246 us 6.7619 us 6.8073 us]
/// Day1 - Part1/(default)  time:   [249.18 ns 250.33 ns 251.93 ns]
/// Day1 - Part2/(default)  time:   [170.63 ns 175.55 ns 181.44 ns]

#[aoc_generator(day1)]
pub fn generate(input: &str) -> Vec<isize> {
    let mut array: Vec<isize> = input.lines().map(|i| i.parse().unwrap()).collect();
    array.sort_unstable();
    array
}

#[inline]
fn two_sum(sorted_array: &[isize], target: isize) -> Option<(isize, isize)> {
    let length = sorted_array.len();
    let (mut start, mut end) = (0, length - 1);
    while start < length {
        let sum = sorted_array[start] + sorted_array[end];
        match sum.cmp(&target) {
            Ordering::Equal => {
                return Some((sorted_array[start], sorted_array[end]));
            }
            Ordering::Greater => {
                if end >= 1 {
                    end -= 1;
                } else {
                    break;
                }
            }
            Ordering::Less => {
                start += 1;
            }
        }
        if start > end {
            break;
        }
    }
    None
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[isize]) -> isize {
    let (i, j) = two_sum(input, 2020).unwrap();
    i * j
}

#[inline]
fn three_sum(sorted_array: &[isize], target: isize) -> Option<(isize, isize, isize)> {
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
                Ordering::Greater => {
                    end -= 1;
                }
                Ordering::Less => {
                    start += 1;
                }
            }
        }
    }
    None
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[isize]) -> isize {
    let (i, j, k) = three_sum(&input, 2020).unwrap();
    i * j * k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = generate("1721\n979\n366\n299\n675\n1456");
        assert_eq!(solve_part1(&input), 514579);
    }

    #[test]
    fn test_part2() {
        let input = generate("1721\n979\n366\n299\n675\n1456");
        assert_eq!(solve_part2(&input), 241861950);
    }

    #[test]
    fn test_half() {
        let input = generate("1010\n2020\n0\n299\n670\n14");
        assert_eq!(solve_part1(&input), 0);
    }
}
