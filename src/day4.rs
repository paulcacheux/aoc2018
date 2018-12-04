use std::str::Bytes;
use std::collections::HashMap;

use chrono::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Log {
    BeginShift(i32), // id
    SleepLog(SleepLog),
}

#[derive(Debug, Clone, Copy)]
pub enum SleepLog {
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct TimedSleepLog {
    time: NaiveTime,
    sleep_log: SleepLog,
}

#[derive(Debug)]
pub struct TimedLog {
    date_time: DateTime<Utc>,
    log: Log,
}

#[derive(Debug)]
struct Infos {
    id: i32,
    logs: Vec<TimedSleepLog>,
}

impl Infos {
    fn new() -> Self {
        Infos {
            id: 0,
            logs: Vec::new(),
        }
    }
}

fn parse_timed_log(mut line: Bytes) -> TimedLog {
    let year;
    let month;
    let day;
    let hour;
    let minute;
    scan!(line => "[{}-{}-{} {}:{}]", year, month, day, hour, minute);

    let action: String;
    scan!(line => " {}", action);

    let log = match action.as_str() {
        "falls" => Log::SleepLog(SleepLog::FallsAsleep),
        "wakes" => Log::SleepLog(SleepLog::WakesUp),
        "Guard" => {
            let id;
            scan!(line => "#{}", id);
            Log::BeginShift(id)
        },
        _ => unreachable!()
    };

    let date_time = Utc.ymd(year, month, day).and_hms(hour, minute, 0);

    TimedLog {
        date_time, log
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<TimedLog> {
    input.lines().map(|l| {
        parse_timed_log(l.bytes())
    }).collect()
}

fn get_night_infos(input: &[TimedLog]) -> HashMap<Date<Utc>, Infos> {
    let mut night_infos: HashMap<Date<Utc>, Infos> = HashMap::new();

    for log in input {
        
        let mut date = log.date_time.date();
        if log.date_time.time().hour() >= 22 {
            date = date.succ();
        }

        let infos = night_infos.entry(date).or_insert_with(Infos::new);
        match log.log {
            Log::BeginShift(id) => infos.id = id,
            Log::SleepLog(sleep_log) => infos.logs.push(TimedSleepLog {
                time: log.date_time.time(),
                sleep_log
            }),
        }
    }

    for infos in night_infos.values_mut() {
        infos.logs.sort_by_key(|t| t.time);
    }

    night_infos
}

fn get_scores_hours(night_infos: HashMap<Date<Utc>, Infos>) -> (HashMap<i32, u32>, HashMap<i32, Vec<u32>>) {
    let mut scores: HashMap<i32, u32> = HashMap::new();
    let mut hours: HashMap<i32, Vec<u32>> = HashMap::new();

    for infos in night_infos.values() {
        let current_score = scores.entry(infos.id).or_insert(0);
        let timetable = hours.entry(infos.id).or_insert_with(|| vec![0; 60]);
       
        for pair in infos.logs.chunks(2) {
            let start_minute = pair[0].time.minute();
            let end_minute = pair[1].time.minute();

            let score = end_minute - start_minute;
            *current_score += score;
            for i in start_minute..end_minute {
                *timetable.get_mut(i as usize).unwrap() += 1;
            }
        }
    }
    (scores, hours)
}

fn get_best_hour_for_id(id: i32, hours: &HashMap<i32, Vec<u32>>) -> i32 {
    let mut max_hour = 0;
    let mut max_count = 0;
    for (hour, &count) in hours.get(&id).unwrap().iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_hour = hour;
        }
    }
    max_hour as _
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[TimedLog]) -> i32 {
    let night_infos = get_night_infos(input);

    let (scores, hours) = get_scores_hours(night_infos);

    let &max_id = scores.iter().max_by_key(|kv| kv.1).unwrap().0;
    let max_hour = get_best_hour_for_id(max_id, &hours);

    max_id * max_hour
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[TimedLog]) -> i32 {
    let night_infos = get_night_infos(input);
    let (_, hours) = get_scores_hours(night_infos);

    let (&id, _) = hours.iter().max_by_key(|kv| kv.1.iter().max()).unwrap();
    let max_hour = get_best_hour_for_id(id, &hours);

    id * max_hour
}