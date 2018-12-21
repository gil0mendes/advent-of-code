extern crate chrono;

use chrono::{NaiveDateTime, Timelike};
use std::collections::HashMap;

const DATA: &str = include_str!("./input.txt");

fn main() {
    let mut records = Vec::new();

    // [1518-11-05 00:55] wakes up
    DATA.lines().for_each(|entry| {
        let (date, rec_type) = entry.split_at(18);
        let date = NaiveDateTime::parse_from_str(date, "[%Y-%m-%d %H:%M]").unwrap();
        records.push((date, rec_type.trim()));
    });

    // Sort the logs by cronological order
    records.sort_by(|prev, cur| prev.0.cmp(&cur.0));

    let mut current_guard = 0u32;
    let mut guard_asleep: Option<NaiveDateTime> = None;

    let mut guard_sleep_freq: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    for (date, record) in records {
        // Record the guard that begin the shift
        if record.ends_with("begins shift") {
            current_guard = str::parse(&record.split(" ").nth(1).unwrap()[1..]).unwrap();
            guard_asleep = None;
            continue;
        }

        match record {
            "falls asleep" => {
                guard_asleep = Some(date);
            }
            "wakes up" => {
                // Compute the number of minutes that the guard was sleeping and
                // sum it to the hash map.
                let asleep_time = guard_asleep.expect("Invalid event");
                let init_min = asleep_time.minute();
                let end_min = date.minute();

                // Register the minutes that the guards was asleep
                for minute in init_min..end_min {
                    *guard_sleep_freq
                        .entry(current_guard)
                        .or_default()
                        .entry(minute)
                        .or_default() += 1;
                }
            }
            other => panic!("Invalid log {:?}", other)
        }
    }

    // Part 1
    let (&sleepiest, _) = guard_sleep_freq
        .iter()
        .max_by_key(|&(_, ref freqs)| -> u32 {
            freqs.values().sum()
        })
        .unwrap();

    let minute = guard_sleep_freq[&sleepiest]
        .iter()
        .max_by_key(|&(_, freq)| freq)
        .map(|(&minute, _)| minute)
        .unwrap();

    println!("Sleepiest Guard: {:?}", sleepiest);
    println!("At minute: {:?}", minute);
    println!("Part1: {}", sleepiest * minute);

    // Part 2
    let (guard, (minute, _)) = guard_sleep_freq
        .iter()
        .map(|(&guard, freqs)| -> (u32, (u32, u32)) {
            let (&minute, &count) = freqs
                .iter()
                .max_by_key(|&(_, min)| min)
                .unwrap();

            (guard, (minute, count))
        })
        .max_by_key(|&(_, freq)| freq.1)
        .unwrap();

    println!("Part 2: {}", guard * minute);
}
