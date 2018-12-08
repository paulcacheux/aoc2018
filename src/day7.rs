use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Requirement {
    pre: char,
    post: char,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Requirement> {
    input.lines().map(|l| {
        let pre;
        let post;
        scan!(l.bytes() => "Step {} must be finished before step {} can begin.", pre, post);
        Requirement { pre, post }
    }).collect()
}

fn get_predecessors(req: &[Requirement]) -> HashMap<char, Vec<char>> {
    let mut predecessors = HashMap::new();

    for r in req {
        predecessors.entry(r.pre).or_insert_with(Vec::new);
        predecessors.entry(r.post).or_insert_with(Vec::new).push(r.pre);
    }
    predecessors
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Requirement]) -> String {
    let mut predecessors = get_predecessors(input);
    let mut output = String::new();
    while !predecessors.is_empty() {
        let mut empty_keys: Vec<_> = predecessors.iter().filter_map(|(k, v)| {
            if v.is_empty() {
                Some(*k)
            } else {
                None
            }
        }).collect();
        empty_keys.sort();

        let choosen_key = empty_keys[0];

        for v in predecessors.values_mut() {
            v.retain(|&x| x != choosen_key);
        }
        predecessors.remove(&choosen_key);
        output.push(choosen_key);
    }
    output
}

#[derive(Debug, Clone, Copy)]
struct Worker {
    time: i32,
    working_on: Option<char>
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Requirement]) -> i32 {
    const BASE_TIME: i32 = 60;
    const N_WORKER: usize = 5;

    let mut workers = vec![Worker { time: 0, working_on: None }; N_WORKER];

    let mut predecessors = get_predecessors(input);
    let mut time = 0;
    loop {
        for w in workers.iter_mut() {
            if w.time == 0 {
                if let Some(k) = w.working_on.take() {
                    for v in predecessors.values_mut() {
                        v.retain(|&x| x != k);
                    }
                    predecessors.remove(&k);
                }
            }
        }

        {
            let current_workers_key: HashSet<char> = workers.iter().filter_map(|w| w.working_on.clone()).collect();
            let mut available_workers: Vec<&mut Worker> = workers.iter_mut().filter(|w| w.working_on.is_none()).collect();
            if !available_workers.is_empty() {
                let mut empty_keys: Vec<_> = predecessors.iter().filter_map(|(k, v)| {
                    if !current_workers_key.contains(k) && v.is_empty() {
                        Some(*k)
                    } else {
                        None
                    }
                }).collect();
                empty_keys.sort();

                for (i, k) in empty_keys.into_iter().take(available_workers.len()).enumerate() {
                    let worker = &mut available_workers[i];
                    worker.time = BASE_TIME + (k as u8 - b'A' + 1) as i32;
                    worker.working_on = Some(k);
                }
            }
        }

        for w in workers.iter_mut() {
            if w.working_on.is_some() {
                w.time -= 1;
            }
        }
        
        if predecessors.is_empty() {
            break;
        }

        time += 1;
    }

    time
}

#[cfg(test)]
mod tests {
    use super::*;
static INPUT: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn part1() {
        let requirements = input_generator(INPUT);
        let result = solve_part1(&requirements);
        assert_eq!(result, "CABDFE");
    }

/* Can't be uncommented because of the constants that are different between test and aoc
    #[test]
    fn part2() {
        let requirements = input_generator(INPUT);
        let result = solve_part2(&requirements);
        assert_eq!(result, 15);
    }
*/
}