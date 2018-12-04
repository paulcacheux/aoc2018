use std::collections::HashMap;
use std::collections::HashSet;

pub struct Claim {
    id: i32,
    offset_x: i32,
    offset_y: i32,
    width: i32,
    height: i32,
}

fn parse_claim(line: &str) -> Claim {
    let mut parts = line.split(|c| c == ' ' || c == '#' || c == '@' || c == ':' || c == ',' || c == 'x');
    parts.next(); // skip
    let id = parts.next().unwrap().parse().unwrap();
    parts.next();
    parts.next();
    let offset_x = parts.next().unwrap().parse().unwrap();
    let offset_y = parts.next().unwrap().parse().unwrap();
    parts.next();
    let width = parts.next().unwrap().parse().unwrap();
    let height = parts.next().unwrap().parse().unwrap();

    Claim { id, offset_x, offset_y, width, height }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    input.lines().map(|l| {
        parse_claim(l)
    }).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Claim]) -> i32 {
    let mut world = HashMap::new();

    let mut counter = 0;

    for claim in input {
        for y in 0..claim.height {
            let y = claim.offset_y + y;
            for x in 0..claim.width {
                let x = claim.offset_x + x;

                *world.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    for v in world.values() {
        if *v >= 2 {
            counter += 1;
        }
    }

    counter
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Claim]) -> i32 {
    let mut world = HashMap::new();

    let mut ids = HashSet::new();
    let mut not_complete_ids = HashSet::new();

    for claim in input {
        ids.insert(claim.id);
        for y in 0..claim.height {
            let y = claim.offset_y + y;
            for x in 0..claim.width {
                let x = claim.offset_x + x;
                world.entry((x, y)).or_insert_with(Vec::new).push(claim.id);
            }
        }
    }

    for v in world.values() {
        if v.len() >= 2 {
            for &id in v.iter() {
                not_complete_ids.insert(id);
            }
        }
    }

    let mut diff = ids.difference(&not_complete_ids);
    *diff.next().unwrap()
}