use std::collections::{HashMap, HashSet};

pub struct Coord {
    x: i32,
    y: i32,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let x;
            let y;
            scan!(line.bytes() => "{}, {}", x, y);
            Coord { x, y }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Coord]) -> i32 {
    let width = (input.iter().max_by_key(|c| c.x).unwrap().x + 1) as usize;
    let height = (input.iter().max_by_key(|c| c.y).unwrap().y + 1) as usize;

    let mut counter = HashMap::new();
    let mut filtered = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            let distances = input
                .iter()
                .map(|c| (c.x - x as i32).abs() + (c.y - y as i32).abs());
            let mut min = None;
            let mut min_counter = 0;
            for (i, d) in distances.enumerate() {
                if let &Some((_, m)) = &min {
                    if d == m {
                        min_counter += 1;
                    } else if d < m {
                        min = Some((i, d));
                        min_counter = 1;
                    }
                } else {
                    min = Some((i, d));
                    min_counter = 1;
                }
            }

            if min_counter != 1 {
                min = None;
            }

            if let Some((index, _)) = min {
                *counter.entry(index).or_insert(0) += 1;

                if x == 0 || y == 0 || x + 1 == width || y + 1 == height {
                    filtered.insert(index);
                }
            }
        }
    }

    for f in filtered {
        counter.remove(&f);
    }

    dbg!(&counter);
    *counter.values().max().unwrap()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Coord]) -> i32 {
    let width = (input.iter().max_by_key(|c| c.x).unwrap().x + 1) as usize;
    let height = (input.iter().max_by_key(|c| c.y).unwrap().y + 1) as usize;

    let mut counter = 0;

    for y in 0..height {
        for x in 0..width {
            let distances = input
                .iter()
                .map(|c| (c.x - x as i32).abs() + (c.y - y as i32).abs());
            
            let total: i32 = distances.sum();
            if total < 10000 {
                counter += 1;
            }
        }
    }

    counter
}