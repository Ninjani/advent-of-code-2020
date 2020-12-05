/// cargo aoc bench results (includes parsing):
/// Day5 - Part1/(default)  time:   [6.5198 us 6.6642 us 6.8384 us]                                                                                               
/// Day5 - Part2/(default)  time:   [6.8722 us 6.9843 us 7.1139 us]                                    

#[inline(always)]
fn get_id(seat: &[u8]) -> u32 {
    seat.iter().take(10).fold(0, |acc, &b| {
        (acc << 1) | if b == b'F' || b == b'L' { 0 } else { 1 }
    })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[u8]) -> u32 {
    input.chunks(11).fold(0, |max, seat| max.max(get_id(&seat)))
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[u8]) -> u32 {
    let (sum, min, max) = input
        .chunks(11)
        .fold((0, 1024, 0), |(sum, min, max), seat| {
            let id = get_id(&seat);
            (sum + id, min.min(id), max.max(id))
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
