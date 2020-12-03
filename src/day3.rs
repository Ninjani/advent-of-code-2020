// use rayon::prelude::*;

/// cargo aoc bench results
/// Generator Day3/(default) time:   [29.794 us 31.944 us 35.957 us]
/// Day3 - Part1/(default)  time:   [1.9827 us 1.9909 us 2.0002 us]                                    
/// Day3 - Part2/(default)  time:   [8.9747 us 9.0545 us 9.1651 us]                                    

#[aoc_generator(day3)]
pub fn generate(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.bytes().map(|c| c == b'#').collect())
        .collect()
}

#[inline(always)]
fn check_slope(input: &[Vec<bool>], slope_i: usize, slope_j: usize) -> usize {
    let (max_i, max_j) = (input[0].len(), input.len());
    (slope_j..=max_j - slope_j)
        .step_by(slope_j)
        .zip((slope_i..).step_by(slope_i))
        .filter(|(index_j, index_i)| input[*index_j][*index_i % max_i])
        .count()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<bool>]) -> usize {
    check_slope(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<bool>]) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(slope_i, slope_j)| check_slope(input, slope_i, slope_j))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = generate("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#");
        assert_eq!(solve_part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = generate("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#");
        assert_eq!(solve_part2(&input), 336);
    }
}
