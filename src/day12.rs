use crate::parse_usize;
use std::mem::swap;

/// cargo aoc bench results
/// Day12 - Part1/(default) time:   [7.6757 us 7.6953 us 7.7185 us]
/// Day12 - Part2/(default) time:   [8.1405 us 8.2008 us 8.2736 us]

#[aoc(day12, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    let mut current_direction = 0;
    let (mut movement_north, mut movement_east) = (0, 0);
    for line in input.split(|&b| b == b'\n') {
        let amount = parse_usize(&line[1..]) as isize;
        match line[0] {
            b'N' => movement_north += amount,
            b'S' => movement_north -= amount,
            b'E' => movement_east += amount,
            b'W' => movement_east -= amount,
            b'F' => match current_direction {
                0 => movement_east += amount,
                90 => movement_north += amount,
                180 => movement_east -= amount,
                270 => movement_north -= amount,
                _ => panic!("unknown direction"),
            },
            b'L' => {
                current_direction += amount;
                current_direction %= 360;
            }
            b'R' => {
                current_direction += 360 - amount;
                current_direction %= 360;
            }
            _ => panic!("unknown direction"),
        }
    }
    (movement_north.abs() as usize) + (movement_east.abs() as usize)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    let (mut waypoint_north, mut waypoint_east) = (1, 10);
    let (mut ship_north, mut ship_east) = (0, 0);
    for line in input.split(|&b| b == b'\n') {
        let amount = parse_usize(&line[1..]) as isize;
        match line[0] {
            b'N' => waypoint_north += amount,
            b'S' => waypoint_north -= amount,
            b'E' => waypoint_east += amount,
            b'W' => waypoint_east -= amount,
            b'F' => {
                ship_north += waypoint_north * amount;
                ship_east += waypoint_east * amount;
            }
            b'R' => {
                for _ in 0..amount / 90 {
                    swap(&mut waypoint_north, &mut waypoint_east);
                    waypoint_north *= -1;
                }
            }
            b'L' => {
                for _ in 0..amount / 90 {
                    swap(&mut waypoint_north, &mut waypoint_east);
                    waypoint_east *= -1;
                }
            }
            _ => panic!("unknown direction"),
        }
    }
    (ship_north.abs() as usize) + (ship_east.abs() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = br#"F10
N3
F7
R90
F11"#;
        assert_eq!(solve_part1(input), 25);
    }

    #[test]
    fn test_part2() {
        let input = br#"F10
N3
F7
R90
F11"#;
        assert_eq!(solve_part2(input), 286);
    }
}
