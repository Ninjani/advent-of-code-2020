#[aoc(day6, part1)]
pub fn solve_part1(input: &[u8]) -> u32 {
    let (total, group, _) =
        input
            .iter()
            .fold((0, 0u32, input[0]), |(mut total, mut group, prev), c| {
                if c != &b'\n' {
                    group |= 1 << (c - b'a');
                } else if prev == b'\n' {
                    total += group.count_ones();
                    group = 0;
                }
                (total, group, *c)
            });
    total + group.count_ones()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[u8]) -> u32 {
    let union_all: u32 = (b'a'..=b'z').fold(0, |union, i| union | (1 << (i - b'a')));
    let (total, group, person, _) = input.iter().fold(
        (0, union_all, 0, input[0]),
        |(mut total, mut group, mut person, prev), c| {
            if c != &b'\n' {
                person |= 1 << (c - b'a');
            } else if prev != b'\n' {
                group &= person;
                person = 0;
            } else {
                total += group.count_ones();
                group = union_all;
            }
            (total, group, person, *c)
        },
    );
    total + (group & person).count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(solve_part1(input.as_bytes()), 11);
    }

    #[test]
    fn test_part2() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(solve_part2(input.as_bytes()), 6);
    }
}
