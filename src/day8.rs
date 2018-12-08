#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn build_tree<I: Iterator<Item=i32>>(iter: &mut I) -> Node {
    let child_count = iter.next().unwrap();
    let metadata_count = iter.next().unwrap();
    let mut childs = Vec::new();
    for _ in 0..child_count {
        childs.push(build_tree(iter));
    }
    let mut metadata = Vec::new();
    for _ in 0..metadata_count {
        metadata.push(iter.next().unwrap());
    }
    Node { child_count, metadata_count, childs, metadata }
}

#[derive(Debug)]
struct Node {
    child_count: i32,
    metadata_count: i32,
    childs: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn sum_metadata(&self) -> i32 {
        let perso: i32 = self.metadata.iter().sum();
        let other: i32 = self.childs.iter().map(Node::sum_metadata).sum();
        perso + other
    }

    fn value(&self) -> i32 {
        if self.child_count == 0 {
            self.metadata.iter().sum()
        } else {
            let mut value = 0;
            for m in &self.metadata {
                let index = m - 1;
                if index >= 0 && index < self.child_count {
                    value += self.childs[index as usize].value();
                }
            }
            value
        }
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let mut iter = input.into_iter().cloned();
    let main_node = build_tree(&mut iter);
    main_node.sum_metadata()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut iter = input.into_iter().cloned();
    let main_node = build_tree(&mut iter);
    main_node.value()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn part1() {
        let numbers = input_generator(INPUT);
        let result = solve_part1(&numbers);
        assert_eq!(result, 138);
    }

    #[test]
    fn part2() {
        let numbers = input_generator(INPUT);
        let result = solve_part2(&numbers);
        assert_eq!(result, 66);
    }
}