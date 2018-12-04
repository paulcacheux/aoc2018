use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.trim().to_string()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(ids: &[String]) -> i32 {
    let mut c2 = 0;
    let mut c3 = 0;
    for id in ids {
        let mut counter = HashMap::new();
        for c in id.chars() {
            *counter.entry(c).or_insert(0i32) += 1;
        }

        let mut b3 = false;
        let mut b2 = false;
        for v in counter.values() {
            if *v == 3 {
                b3 = true;
            } else if *v == 2 {
                b2 = true;
            }
        }
        c2 += if b2 { 1 } else { 0 };
        c3 += if b3 { 1 } else { 0 };
    }
    c2 * c3
}

#[aoc(day2, part2)]
pub fn solve_part2(ids: &[String]) -> String {
    fn compare(id1: &str, id2: &str) -> (i32, String) {
        let mut result = String::new();
        let mut diff_count = 0;

        for (c1, c2) in id1.chars().zip(id2.chars()) {
            if c1 == c2 {
                result.push(c1);
            } else {
                diff_count += 1;
            }
        }
        (diff_count, result)
    }

    for id1 in ids {
        for id2 in ids {
            let (c, r) = compare(id1, id2);
            if c == 1 {
                return r;
            }
        }
    }
    unreachable!()
}