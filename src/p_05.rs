use std::fs::read_to_string;

fn read_input(path: &str) -> (Vec<[u64; 2]>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    for line in read_to_string(path).expect("Unable to read file").lines() {
        if line.contains('-') {
            let splits = line.split_once("-").expect("Invalid input line");
            let (start, end) = (
                splits.0.parse::<u64>().expect("Unable to parse number"),
                splits.1.parse::<u64>().expect("Unable to parse number"),
            );
            ranges.push([start, end]);
        } else if line.eq("") {
            continue;
        } else {
            ids.push(line.parse::<u64>().expect("Unable to parse ID to check"));
        }
    }
    return (ranges, ids);
}

fn part1(path: &str) -> u64 {
    let (ranges, ids) = read_input(path);
    let merged_ranges = merge_overlapping_intervals(ranges);
    let mut ans = 0;
    for id in ids {
        if binary_search_ranges(&merged_ranges, id) {
            ans += 1;
        }
    }
    ans
}

fn merge_overlapping_intervals(mut ranges: Vec<[u64; 2]>) -> Vec<[u64; 2]> {
    ranges.sort_by(|a, b| a[0].cmp(&b[0]).then_with(|| a[1].cmp(&b[1])));
    let mut merged: Vec<[u64; 2]> = Vec::new();
    for r in ranges {
        if let Some(last) = merged.last_mut() {
            if r[0] <= last[1] {
                last[1] = last[1].max(r[1]);
                continue;
            }
        }
        merged.push(r);
    }
    merged
}

fn binary_search_ranges(merged: &[[u64; 2]], id: u64) -> bool {
    merged
        .binary_search_by(|range| {
            if id < range[0] {
                std::cmp::Ordering::Greater
            } else if id > range[1] {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

fn part_2(path: &str) -> u64 {
    let (ranges, ids) = read_input(path);
    let merged_ranges = merge_overlapping_intervals(ranges);
    let mut ans = 0;
    for r in merged_ranges {
        ans += r[1] - r[0] + 1;
    }
    ans
}

mod test {
    use crate::p_05::{part_2, part1, read_input};

    #[test]
    fn test_basic() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_05_sample.txt";
        println!("The answer is {}", part1(PATH));
    }

    #[test]
    fn test_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_05.txt";
        println!("The answer is {}", part1(PATH));
    }

    #[test]
    fn test_basic_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_05_sample.txt";
        println!("The answer is {}", part_2(PATH));
    }

    #[test]
    fn test_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_05.txt";
        println!("The answer is {}", part_2(PATH));
    }
}
