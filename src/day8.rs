/// cargo aoc bench results
/// Generator Day8/(default) time:   [29.155 us 29.477 us 29.895 us]
/// Day8 - Part1/(default)  time:   [331.08 ns 333.25 ns 336.36 ns]
/// Day8 - Part2/(default)  time:   [45.053 us 45.797 us 46.683 us]

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

#[aoc_generator(day8)]
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
    let mut seen: Vec<_> = (0..num_instructions).map(|_| false).collect();
    let mut index = 0;
    loop {
        let terminated = index == num_instructions;
        if terminated || seen[index] {
            return (terminated, accumulator);
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
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    run_bootloader(input).1
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Instruction]) -> i32 {
    let mut input = input.to_vec();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = generate(
            r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#,
        );
        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = generate(
            r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#,
        );
        assert_eq!(solve_part2(&input), 8);
    }
}
