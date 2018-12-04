use std::collections::HashSet;

type Offset = i32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Offset> {
    input.lines().map(|l| {
        l.trim().parse().unwrap()
    }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Offset]) -> i32 {
    input.iter().fold(0, |acc, x| acc + x)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Offset]) -> i32 {
    let mut viewed = HashSet::new();
    let mut current_freq = 0;
    viewed.insert(0);
    loop {
        for i in input {
            current_freq += *i;
            if viewed.contains(&current_freq) {
                return current_freq;
            } else {
                viewed.insert(current_freq);
            }
        }
    }
    
}