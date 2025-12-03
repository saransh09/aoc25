use std::{fs::read_to_string, u32};

fn read_input(path: &str) -> Vec<Vec<u32>> {
    let mut inputs = Vec::new();
    for line in read_to_string(path).expect("Unable to read input").lines() {
        let mut bank: Vec<u32> = Vec::new();
        for ch in line.chars() {
            bank.push(ch.to_digit(10).expect("Bad digit"));
        }
        inputs.push(bank);
    }
    inputs
}

fn pr_1(path: &str) -> u64 {
    let banks = read_input(path);
    let mut answer: u64 = 0;
    for bank in banks.into_iter() {
        let jolts = get_max_joltage(bank) as u64;
        answer += jolts;
    }
    answer
}

fn get_max_joltage(bank: Vec<u32>) -> u32 {
    let mut lmax = 0;
    let mut li = 0;
    for b in 0..bank.len() - 1 {
        if lmax < bank[b] {
            lmax = bank[b];
            li = b;
        }
    }
    let mut rmax = 0;
    for b in li + 1..bank.len() {
        if rmax < bank[b] {
            rmax = bank[b];
        }
    }
    lmax * 10 + rmax
}

fn pr_2(path: &str) -> u128 {
    let banks = read_input(path);
    let mut answer: u128 = 0;
    for bank in banks.into_iter() {
        answer += get_max_joltage_2(bank);
    }
    answer
}

fn get_max_joltage_2(bank: Vec<u32>) -> u128 {
    let digits_remaining: usize = 12;
    let mut max_joltage: u128 = 0;
    let mut li = 0;
    for r in (1..=digits_remaining).rev() {
        let ri = bank.len() - (r - 1);
        let (max_dig_, new_li) = find_max_digit(&bank, li, ri);
        let mut max_dig = max_dig_ as u128;
        li = new_li + 1;
        for _ in 1..r {
            max_dig *= 10;
        }
        max_joltage += max_dig;
    }
    return max_joltage;
}

fn find_max_digit(bank: &Vec<u32>, li: usize, ri: usize) -> (u32, usize) {
    let mut new_li = 0;
    let mut max_dig = 0;
    for b in li..ri {
        if max_dig < bank[b] {
            max_dig = bank[b];
            new_li = b;
        }
    }
    (max_dig, new_li)
}

mod test {
    use crate::p_03::{pr_1, pr_2, read_input};

    #[test]
    fn test_sample_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_03_sample.txt";
        let inputs = read_input(PATH);
        for bank in inputs {
            for bat in bank {
                print!("{},", bat);
            }
            println!();
        }
        println!("The answer is {}", pr_1(PATH));
    }

    #[test]
    fn test_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_03.txt";
        println!("The answer is {}", pr_1(PATH));
    }

    #[test]
    fn test_sample_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_03_sample.txt";
        println!("The answer is {}", pr_2(PATH));
    }

    #[test]
    fn test_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_03.txt";
        println!("The answer is {}", pr_2(PATH));
    }
}
