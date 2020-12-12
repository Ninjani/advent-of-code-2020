use itertools::Itertools;

#[derive(Clone)]
pub struct Grid {
    items: Vec<Option<bool>>,
    num_rows: usize,
    num_cols: usize,
}

impl Grid {
    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.items[x * self.num_cols + y]
    }
    #[inline(always)]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Option<bool> {
        &mut self.items[x * self.num_cols + y]
    }
    #[inline(always)]
    pub fn get_option(&self, x_y: Option<(usize, usize)>) -> Option<bool> {
        if let Some((x, y)) = x_y {
            self.get(x, y)
        } else {
            None
        }
    }
}

#[inline(always)]
pub fn generate(input: &[u8]) -> Grid {
    let num_cols = memchr::memchr(b'\n', input).unwrap();
    let items: Vec<_> = input
        .iter()
        .filter_map(|&c| match c {
            b'L' => Some(Some(false)),
            b'.' => Some(None),
            b'\n' => None,
            _ => panic!("unknown character"),
        })
        .collect();
    let num_rows = items.len() / num_cols;
    Grid {
        items,
        num_rows,
        num_cols,
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    let mut input = generate(input);
    let mut changed;
    let mut neighbors = Vec::with_capacity(input.items.len());
    loop {
        changed = false;
        for i in 0..input.num_rows {
            for j in 0..input.num_cols {
                neighbors.push(
                    (if i == 0 { i } else { i - 1 }..(i + 2).min(input.num_rows))
                        .cartesian_product(
                            if j == 0 { j } else { j - 1 }..(j + 2).min(input.num_cols),
                        )
                        .map(|(x, y)| input.get(x, y).unwrap_or(false) as usize)
                        .sum(),
                );
            }
        }
        input
            .items
            .iter_mut()
            .zip(neighbors.drain(..))
            .for_each(|(i, n)| {
                *i = match (&i, n) {
                    (Some(false), 0) => {
                        changed = true;
                        Some(true)
                    }
                    (Some(true), n) if n >= 5 => {
                        changed = true;
                        Some(false)
                    }
                    _ => *i,
                }
            });
        if !changed {
            break;
        }
    }
    input.items.iter().filter(|&&x| x == Some(true)).count()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    let mut input = generate(input);
    let mut changed;
    let mut neighbors = Vec::with_capacity(input.items.len());
    loop {
        changed = false;
        for i in 0..input.num_rows {
            for j in 0..input.num_cols {
                neighbors.push(
                    vec![
                        ((i + 1)..input.num_rows)
                            .find(|&x| input.get(x, j).is_some())
                            .zip(Some(j)),
                        (0..i)
                            .rev()
                            .find(|&x| input.get(x, j).is_some())
                            .zip(Some(j)),
                        Some(i).zip(((j + 1)..input.num_cols).find(|&x| input.get(i, x).is_some())),
                        Some(i).zip((0..j).rev().find(|&x| input.get(i, x).is_some())),
                        (0..i)
                            .rev()
                            .zip((0..j).rev())
                            .take(i.min(j))
                            .find(|(x, y)| input.get(*x, *y).is_some()),
                        ((i + 1)..input.num_rows)
                            .zip((0..j).rev())
                            .take((input.num_rows - i).min(j))
                            .find(|(x, y)| input.get(*x, *y).is_some()),
                        (0..i)
                            .rev()
                            .zip((j + 1)..input.num_cols)
                            .take(i.max(input.num_cols - j))
                            .find(|(x, y)| input.get(*x, *y).is_some()),
                        ((i + 1)..input.num_rows)
                            .zip((j + 1)..input.num_cols)
                            .take((input.num_rows - i).max(input.num_cols - j))
                            .find(|(x, y)| input.get(*x, *y).is_some()),
                    ]
                    .into_iter()
                    .map(|x| input.get_option(x).unwrap_or(false) as usize)
                    .sum(),
                );
            }
        }
        input
            .items
            .iter_mut()
            .zip(neighbors.drain(..))
            .for_each(|(i, n)| {
                *i = match (&i, n) {
                    (Some(false), 0) => {
                        changed = true;
                        Some(true)
                    }
                    (Some(true), n) if n >= 5 => {
                        changed = true;
                        Some(false)
                    }
                    _ => *i,
                }
            });
        if !changed {
            break;
        }
    }
    input.items.iter().filter(|&&x| x == Some(true)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = br#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        assert_eq!(solve_part1(input), 37);
    }

    #[test]
    fn test_part2() {
        let input = br#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        assert_eq!(solve_part2(input), 26);
    }
}
