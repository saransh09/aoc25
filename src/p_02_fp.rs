use std::fs::read_to_string;

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
            let (start, end) = v
                .trim()
                .split_once('-')
                .expect("Each range should contain exactly one '-'");
            Range {
                start: start.parse::<u128>().expect("Bad start number"),
                end: end.parse::<u128>().expect("Bad end number"),
            }
        })
        .collect()
}

fn part1(path: &str) -> u128 {
    read_input(path)
        .into_iter()
        .map(|range| invalid_sum(range, is_invalid))
        .sum()
}

fn part2(path: &str) -> u128 {
    read_input(path)
        .into_iter()
        .map(|range| invalid_sum(range, is_invalid_2))
        .sum()
}

fn invalid_sum(range: Range, predicate: fn(u128) -> bool) -> u128 {
    (range.start..=range.end)
        .filter(|&n| predicate(n))
        .sum::<u128>()
}

fn is_invalid(num: u128) -> bool {
    let s = num.to_string();
    let len = s.len();

    (len % 2 == 0) && {
        let m = len / 2;
        &s[..m] == &s[m..]
    }
}

fn is_invalid_2(num: u128) -> bool {
    let s = num.to_string();
    let len = s.len();

    (1..=len / 2).filter(|k| len % k == 0).any(|k| {
        let chunk = &s[..k];
        (k..len).step_by(k).all(|i| &s[i..i + k] == chunk)
    })
}

mod test {
    use super::{part1, part2};

    #[test]
    fn test_sample() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_02_sample.txt";
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
