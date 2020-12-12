use crate::parse_usizes;
use itertools::Itertools;
use std::cmp::Ordering;

/// cargo aoc bench results
/// Day9 - Part1/(default)  time:   [30.626 us 31.682 us 33.012 us]    
/// Day9 - Part2/(default)  time:   [553.12 ns 560.96 ns 569.68 ns] (without parsing and Part 1)
/// Day9 - Part2/prefix_sum time:   [1.7565 us 1.7764 us 1.7990 us] (without parsing and Part 1)

#[inline(always)]
fn first_wrong_solve(numbers: &[usize], preamble_size: usize) -> usize {
    for (window, next_value) in numbers
        .windows(preamble_size)
        .zip(numbers.iter().skip(preamble_size))
    {
        if !window
            .iter()
            .enumerate()
            .any(|(i, v)| next_value > v && window[i + 1..].contains(&(next_value - v)))
        {
            return *next_value;
        }
    }
    unreachable!()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    let input = parse_usizes(input);
    first_wrong_solve(&input, 25)
}

#[inline(always)]
fn sliding_window_solve(input: &[usize], sum: usize) -> usize {
    let mut current_sum = input[0];
    let length = input.len();
    let (mut start, mut end) = (0, 1);
    loop {
        while current_sum > sum && start < end - 1 {
            current_sum -= input[start];
            start += 1;
        }
        if current_sum == sum {
            let (min, max) = input[start..end].iter().minmax().into_option().unwrap();
            return min + max;
        }
        while current_sum < sum && end < length - 1 {
            current_sum += input[end];
            end += 1;
        }
    }
}

#[inline(always)]
fn prefix_sum_solve(input: &[usize], sum: usize) -> usize {
    let length = input.len();
    let mut prefix_sum = vec![0; length];
    let mut prev_prefix = 0;
    for i in 0..length {
        prev_prefix += input[i];
        prefix_sum[i] = prev_prefix;
    }
    let (mut start, mut end) = (0, 1);
    while start < length && end < length {
        let difference = prefix_sum[end] - prefix_sum[start];
        match (difference.cmp(&sum), start == end) {
            (Ordering::Equal, false) => break,
            (Ordering::Less, _) => end += 1,
            _ => start += 1,
        }
    }
    let (min, max) = input[start..end].iter().minmax().into_option().unwrap();
    min + max
}

fn get_input() -> &'static [u8] {
    include_bytes!("../input/2020/day9.txt")
}

#[ctor::ctor]
static PART_1: usize = solve_part1(get_input());
#[ctor::ctor]
static PART_2_INPUT: Vec<usize> = parse_usizes(get_input());

#[aoc(day9, part2)]
pub fn solve_part2(_: &[u8]) -> usize {
    // let input = parse_usizes(input);
    sliding_window_solve(&PART_2_INPUT, *PART_1)
}

#[aoc(day9, part2, prefix_sum)]
pub fn solve_part2_prefix_sum(_: &[u8]) -> usize {
    // let input = parse_usizes(input);
    prefix_sum_solve(&PART_2_INPUT, *PART_1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_usizes(
            br#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#,
        );
        assert_eq!(first_wrong_solve(&input, 5), 127);
    }

    #[test]
    fn test_part2() {
        let input = parse_usizes(
            br#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#,
        );
        let wrong_number = first_wrong_solve(&input, 5);
        assert_eq!(sliding_window_solve(&input, wrong_number), 62);
        assert_eq!(prefix_sum_solve(&input, wrong_number), 62);
    }
}
