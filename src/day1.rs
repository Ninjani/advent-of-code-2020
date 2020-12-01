use std::cmp::Ordering;

#[aoc_generator(day1)]
pub fn generate(input: &str) -> Vec<isize> {
    input.split('\n').map(|i| i.parse().unwrap()).collect()
}

#[inline]
fn two_sum(array: &[isize], target: isize) -> Option<(isize, isize)> {
    let mut sorted_array: Vec<_> = array.iter().cloned().collect();
    sorted_array.sort();
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
fn three_sum(array: &[isize], target: isize) -> Option<(isize, isize, isize)> {
    let mut sorted_array: Vec<_> = array.iter().cloned().collect();
    sorted_array.sort();
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
}
