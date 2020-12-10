/// cargo aoc bench results
/// Day3 - Part1/(default)  time:   [27.890 us 28.047 us 28.248 us]
/// Day3 - Part2/(default)  time:   [41.501 us 42.095 us 42.740 us]                                  

#[inline(always)]
pub fn generate(input: &[u8]) -> Vec<Vec<bool>> {
    input
        .split(|&b| b == b'\n')
        .map(|line| line.iter().map(|&c| c == b'#').collect())
        .collect()
}

#[inline(always)]
fn check_slope(input: &[Vec<bool>], slope_i: usize, slope_j: usize) -> usize {
    let (max_i, max_j) = (input[0].len(), input.len());
    (slope_j..max_j - slope_j + 1)
        .step_by(slope_j)
        .enumerate()
        .filter(|(index_i, index_j)| input[*index_j][((index_i + 1) * slope_i) % max_i])
        .count()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    let input = generate(input);
    check_slope(&input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    let input = generate(input);
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(slope_i, slope_j)| check_slope(&input, slope_i, slope_j))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = b"..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(solve_part1(input), 7);
    }

    #[test]
    fn test_part2() {
        let input = b"..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(solve_part2(input), 336);
    }
}
