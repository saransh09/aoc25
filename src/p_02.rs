use std::{fs::read_to_string, num};

#[derive(Debug)]
struct Range {
    start: u128,
    end: u128,
}

fn read_input(path: &str) -> Vec<Range> {
    read_to_string(path)
        .expect("Unable to open file")
        .split(',')
        .map(|v| {
            let mut parts = v.trim().split('-');
            let start = parts
                .next()
                .expect("Unable to fetch the start range string")
                .parse::<u128>()
                .expect("Unable to convert start range to u128");
            let end = parts
                .next()
                .expect("Unable to fetch the end range string")
                .parse::<u128>()
                .expect("Unable to convert end range to u128");
            Range {
                start: start,
                end: end,
            }
        })
        .collect()
}

fn get_invalid_ids(r: Range) -> u128 {
    let mut invalid_id_sum = 0;
    for i in r.start..=r.end {
        if is_invalid(i) {
            invalid_id_sum += i;
        }
    }
    invalid_id_sum
}

fn is_invalid(num: u128) -> bool {
    let s = num.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let m = s.len() / 2;
    if &s[0..m] == &s[m..] {
        return true;
    }
    false
}

fn part1(path: &str) -> u128 {
    let ranges = read_input(path);
    let mut invalid_sum: u128 = 0;
    for range in ranges {
        invalid_sum += get_invalid_ids(range);
    }
    invalid_sum
}

fn part2(path: &str) -> u128 {
    let ranges = read_input(path);
    let mut invalid_sum: u128 = 0;
    for range in ranges {
        invalid_sum += get_invalid_ids_2(range);
    }
    invalid_sum
}

fn get_invalid_ids_2(r: Range) -> u128 {
    let mut invalid_id_sum = 0;
    for i in r.start..=r.end {
        if is_invalid_2(i) {
            invalid_id_sum += i;
        }
    }
    invalid_id_sum
}

fn is_invalid_2(num: u128) -> bool {
    // println!("Processing number : {}", num);
    let s = num.to_string();
    let l = s.len();
    for k in 1..=l / 2 {
        if l % k != 0 {
            continue;
        }
        let first = &s[0..k];
        let mut matched = true;
        for i in (k..l).step_by(k) {
            if &s[i..i + k] != first {
                matched = false;
                break;
            }
        }
        if matched {
            // println!("Matched!!!");
            return true;
        }
    }
    // println!("Not Matched!!!");
    // println!();
    false
}

mod test {
    use crate::p_02::{part1, part2, read_input};

    #[test]
    fn test_sample() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_02_sample.txt";
        // let ranges = read_input(PATH);
        // for range in ranges {
        //     println!("{:?}", range);
        // }
        println!("{}", part1(PATH));
    }

    #[test]
    fn test_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_02.txt";
        println!("{}", part1(PATH));
    }

    #[test]
    fn test_sample_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_02_sample.txt";
        println!("{}", part2(PATH));
    }

    #[test]
    fn test_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_02.txt";
        println!("{}", part2(PATH));
    }
}
