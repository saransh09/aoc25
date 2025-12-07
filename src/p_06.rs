use std::{
    fs::{File, read_to_string},
    io::{BufRead, BufReader},
    usize,
};

fn read_input(path: &str) -> (Vec<Vec<u128>>, Vec<char>) {
    let mut numbers: Vec<Vec<u128>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();

    let file = File::open(path).expect("Unable to read file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines().peekable();

    while let Some(line) = lines.next() {
        let line = line.expect("Unable to read line");
        if lines.peek().is_none() {
            operators = _read_operators(&line);
        } else {
            numbers.push(_read_num_line(&line));
        }
    }

    return (numbers, operators);
}

fn _read_num_line(line: &str) -> Vec<u128> {
    line.split_whitespace()
        .map(|c| c.parse::<u128>().expect("Unable to convert to u128"))
        .collect()
}

fn _read_operators(line: &str) -> Vec<char> {
    line.split_whitespace()
        .map(|s| s.chars().next().expect("No operator found"))
        .collect()
}

fn part1(path: &str) -> u128 {
    let (numbers, operators) = read_input(path);
    let mut ans = vec![0u128; operators.len()];
    for i in 0..operators.len() {
        let op = operators[i];
        if op == '*' {
            ans[i] = 1;
        }

        if op == '+' {
            for num in numbers.iter() {
                ans[i] += num[i];
            }
        } else if op == '*' {
            for num in numbers.iter() {
                ans[i] *= num[i];
            }
        }
    }
    return ans.iter().sum();
}

fn read_input_2(path: &str) -> (Vec<Vec<String>>, Vec<char>) {
    let mut operators = Vec::new();
    let mut rows = Vec::new();
    let file = File::open(path).expect("Unable to read file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().peekable();

    while let Some(line) = lines.next() {
        let line = line.expect("Unable to read line");
        if lines.peek().is_none() {
            operators = _read_operators(&line);
        } else {
            rows.push(_read_num_line_2(&line));
        }
    }
    let height = rows.len();
    let width = rows[0].len();

    let mut cols = vec![Vec::with_capacity(height); width];

    for row in &rows {
        for (col_idx, val) in row.iter().enumerate() {
            cols[col_idx].push(val.clone());
        }
    }

    cols.reverse();
    operators.reverse();

    (cols, operators)
}

fn _read_num_line_2(line: &str) -> Vec<String> {
    line.split_whitespace().map(|n| n.to_string()).collect()
}

fn part2(path: &str) -> u128 {
    let (mut cols, mut ops) = read_input_2(path);

    let mut paired: Vec<(Vec<String>, char)> = cols.into_iter().zip(ops.into_iter()).collect();

    paired.reverse();

    let mut ans = Vec::new();

    for (nums_col, op) in paired {
        let nums_to_work = return_significant_sum(&nums_col);
        let res = if op == '*' {
            nums_to_work.into_iter().product::<u128>()
        } else {
            nums_to_work.into_iter().sum::<u128>()
        };
        ans.push(res);
    }
    ans.iter().sum()
}

fn return_significant_sum(nums: &Vec<String>) -> Vec<u128> {
    let (max_len, min_len) = _get_digit_bounds(&nums);
    let mut sol: Vec<u128> = Vec::new();
    for pos in (1..=max_len).rev() {
        let mut curr_number = String::new();
        for num in nums {
            if num.len() >= pos {
                curr_number.push(num.chars().nth(pos - 1).expect("Unable to read_char"));
            }
        }

        sol.push(
            curr_number
                .parse::<u128>()
                .expect("Unable to convert to u128"),
        );
    }
    sol
}

fn _get_digit_bounds(nums: &Vec<String>) -> (usize, usize) {
    let mut max_len = 0;
    let mut min_len = usize::MAX;
    for n in nums {
        if n.len() > max_len {
            max_len = n.len();
        }
        if n.len() < min_len {
            min_len = n.len();
        }
    }
    (max_len, min_len)
}

mod test {
    use crate::p_06::{part1, part2, read_input, read_input_2};

    #[test]
    fn test_sample_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_06_sample.txt";
        println!("The answer is : {}", part1(PATH));
    }

    #[test]
    fn test_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_06.txt";
        println!("The answer is : {}", part1(PATH));
    }

    #[test]
    fn test_sample_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_06_sample.txt";
        let (cols, ops) = read_input_2(PATH);
        for col in cols {
            println!("{:?}", col);
        }
        println!("The answer is : {}", part2(PATH));
    }

    #[test]
    fn test_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_06.txt";
        println!("The answer is : {}", part2(PATH));
    }
}
