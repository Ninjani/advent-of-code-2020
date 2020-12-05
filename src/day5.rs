/// cargo aoc bench results:
/// Day5 - Part1/(default)  time:   [8.1343 us 8.2398 us 8.3567 us]                                   
/// Day5 - Part2/(default)  time:   [5.9458 us 6.0279 us 6.1174 us]

#[inline(always)]
fn get_id(seat: &[u8]) -> u32 {
    seat.iter().fold(0, |acc, &b| {
        acc * 2 + if b == b'F' || b == b'L' { 0 } else { 1 } as u32
    })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[u8]) -> u32 {
    (0..input.len() / 11 + 1).fold(0, |max, i| get_id(&input[i * 11..i * 11 + 10]).max(max))
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[u8]) -> u32 {
    let (sum, min, max) = (0..input.len() / 11 + 1).fold((0, 1024, 0), |(sum, min, max), i| {
        let id = get_id(&input[i * 11..i * 11 + 10]);
        (sum + id, id.min(min), id.max(max))
    });
    (min..max + 1).sum::<u32>() - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(b"FBFBBFFRLR"), 357);
        assert_eq!(solve_part1(b"BFFFBBFRRR"), 567);
        assert_eq!(solve_part1(b"BBFFBBFRLL"), 820);
    }
}
