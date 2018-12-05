use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    c: char,
    capitalized: bool,
}

#[aoc_generator(day5)]
pub fn generate_input(input: &str) -> Vec<Unit> {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            let (c, capitalized) = if c.is_lowercase() {
                (c, false)
            } else {
                (c.to_ascii_lowercase(), true)
            };
            Unit { c, capitalized }
        })
        .collect()
}

fn get_final_len(input: &[Unit], filter: Option<char>) -> usize {
    let scanner = input
        .into_iter()
        .filter(|unit| filter.map(|other| unit.c != other).unwrap_or(true))
        .scan(Vec::new(), |state: &mut Vec<Unit>, unit| {
            if let Some(top_pair) = state.pop() {
                if top_pair.c != unit.c || top_pair.capitalized == unit.capitalized {
                    state.push(top_pair);
                    state.push(*unit);
                }
            } else {
                state.push(*unit);
            }

            Some(state.len())
        });

    scanner.last().unwrap()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Unit]) -> usize {
    get_final_len(input, None)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Unit]) -> usize {
    let chars: HashSet<char> = input.iter().map(|u| u.c).collect();

    println!("{:?}", chars);

    chars
        .into_iter()
        .map(|c| get_final_len(input, Some(c)))
        .min()
        .unwrap()
}
