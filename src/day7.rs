use std::collections::{HashMap, HashSet, VecDeque};

/// cargo aoc bench results
/// Day7 - Part1/(default)  time:   [499.47 us 506.25 us 514.46 us]                                   
/// Day7 - Part2/(default)  time:   [512.04 us 526.67 us 544.56 us]

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut graph = HashMap::new();
    input
        .split('\n')
        .filter(|line| !line.ends_with("no other bags."))
        .for_each(|line| {
            let (start_node, end_nodes) = line.split_once(" contain ").unwrap();
            let (start_node, _) = start_node.split_once(" bag").unwrap();
            end_nodes.split(", ").for_each(|end_node| {
                let (_, end_node) = end_node.split_once(' ').unwrap();
                graph
                    .entry(end_node.split(" bag").next().unwrap())
                    .or_insert_with(Vec::new)
                    .push(start_node);
            });
        });
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");
    let mut seen = HashSet::new();
    while let Some(start) = queue.pop_front() {
        if seen.insert(start) {
            if let Some(nodes) = graph.get(start) {
                queue.extend(nodes.iter());
            }
        }
    }
    seen.len() - 1
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut graph = HashMap::new();
    input
        .split('\n')
        .filter(|line| !line.ends_with("no other bags."))
        .for_each(|line| {
            let (start_node, end_nodes) = line.split_once(" contain ").unwrap();
            let (start_node, _) = start_node.split_once(" bag").unwrap();
            end_nodes.split(", ").for_each(|end_node| {
                let (weight, end_node) = end_node.split_once(' ').unwrap();
                graph.entry(start_node).or_insert_with(Vec::new).push((
                    end_node.split(" bag").next().unwrap(),
                    weight.parse::<usize>().unwrap(),
                ));
            });
        });
    let mut queue = VecDeque::new();
    queue.push_back(("shiny gold", 1));
    let mut total = 0;
    while let Some((start, number)) = queue.pop_front() {
        total += number;
        if let Some(edges) = graph.get(start) {
            queue.extend(edges.iter().map(|(end, weight)| (*end, number * weight)));
        }
    }
    total - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        assert_eq!(solve_part1(&input), 4);
    }

    #[test]
    fn test_part2() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        assert_eq!(solve_part2(&input), 32);

        let input = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        assert_eq!(solve_part2(&input), 126);
    }
}
