use std::collections::VecDeque;

/// cargo aoc bench results
/// Day8 - Part1/(default)  time:   [32.372 us 32.584 us 32.814 us]
/// Day8 - Part2/(loop)     time:   [82.817 us 87.361 us 92.198 us]
/// Day8 - Part2/(graph)    time:   [85.462 us 86.598 us 87.912 us]

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

#[inline(always)]
pub fn generate(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|instruction| {
            let (op, num) = instruction.split_once(' ').unwrap();
            let num: i32 = num.parse().unwrap();
            match op {
                "acc" => Instruction::Acc(num),
                "nop" => Instruction::Nop(num),
                "jmp" => Instruction::Jmp(num),
                _ => panic!("unknown instruction"),
            }
        })
        .collect()
}

#[inline]
fn run_bootloader(input: &[Instruction]) -> (bool, i32) {
    let num_instructions = input.len();
    let mut accumulator = 0;
    let mut seen = vec![false; num_instructions];
    let mut index = 0;
    while index != num_instructions {
        if seen[index] {
            return (false, accumulator);
        }
        seen[index] = true;
        match input[index] {
            Instruction::Acc(num) => {
                accumulator += num;
                index += 1;
            }
            Instruction::Nop(_) => index += 1,
            Instruction::Jmp(num) => index = (index as i32 + num) as usize,
        }
    }
    (true, accumulator)
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let input = generate(input);
    run_bootloader(&input).1
}

#[aoc(day8, part2, loop)]
pub fn solve_part2(input: &str) -> i32 {
    let mut input = generate(input);
    for op_index in 0..input.len() {
        match input[op_index] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(num) => input[op_index] = Instruction::Nop(num),
            Instruction::Nop(num) => input[op_index] = Instruction::Jmp(num),
        }
        let (fixed, acc) = run_bootloader(&input);
        if fixed {
            return acc;
        }
        match input[op_index] {
            Instruction::Jmp(num) => input[op_index] = Instruction::Nop(num),
            Instruction::Nop(num) => input[op_index] = Instruction::Jmp(num),
            _ => (),
        }
    }
    unreachable!()
}

#[inline(always)]
fn generate_part2_graph(input: &str) -> Vec<(usize, i32, Option<usize>)> {
    input
        .split('\n')
        .enumerate()
        .map(|(index, instruction)| {
            let (op, num) = instruction.split_once(' ').unwrap();
            let num: i32 = num.parse().unwrap();
            match op {
                "acc" => (index + 1, num, None),
                "nop" => (index + 1, 0, Some((index as i32 + num) as usize)),
                "jmp" => ((index as i32 + num) as usize, 0, Some(index + 1)),
                _ => panic!("unknown instruction"),
            }
        })
        .collect()
}

#[aoc(day8, part2, graph)]
fn solve_part2_graph(input: &str) -> i32 {
    let input = generate_part2_graph(input);
    let end_node = input.len();
    let mut paths = VecDeque::new();
    paths.push_back((0, false, 0));
    while let Some((last_node, flipped, accumulator)) = paths.pop_front() {
        let (next_node, acc, next_node_flipped) = input[last_node];
        if !flipped {
            if let Some(next_node_flipped) = next_node_flipped {
                if next_node_flipped == end_node {
                    return accumulator + acc;
                }
                paths.push_back((next_node_flipped, true, accumulator + acc));
            }
        }
        if next_node == end_node {
            return accumulator + acc;
        }
        paths.push_back((next_node, flipped, accumulator + acc));
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        assert_eq!(solve_part2(&input), 8);
        assert_eq!(solve_part2_graph(&input), 8);
    }
}
