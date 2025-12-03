use std::fs::read_to_string;

struct Rotate {
    dir: char,
    amount: u32,
}

fn read_input(path: &str) -> Vec<Rotate> {
    let mut rotations: Vec<Rotate> = Vec::new();
    for line in read_to_string(path)
        .expect("Unable to read input from path")
        .lines()
    {
        let dir = line.chars().nth(0).expect("Unable to fetch direction");
        let amount = line[1..]
            .parse::<u32>()
            .expect("Unable to convert the amount");
        rotations.push(Rotate {
            dir: dir,
            amount: amount,
        });
    }
    rotations
}

fn mod_pos(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

fn get_password(path: &str) -> i64 {
    let rotations = read_input(path);
    let mut start_amount: i64 = 50;
    let mut password: i64 = 0;
    for rotate in rotations.iter() {
        match rotate.dir {
            'L' => {
                start_amount -= rotate.amount as i64;
                start_amount = mod_pos(start_amount, 100);
                if start_amount == 0 {
                    password += 1;
                }
            }
            'R' => {
                start_amount += rotate.amount as i64;
                start_amount = mod_pos(start_amount, 100);
                if start_amount == 0 {
                    password += 1;
                }
            }
            _ => panic!("Unknown direction"),
        }
    }
    password
}

fn get_password_0x434C49434B(path: &str) -> i64 {
    let rotations = read_input(path);
    let mut start_amount: i64 = 50;
    let mut password: i64 = 0;

    for rotate in rotations.iter() {
        let mut clicks = rotate.amount as i64;

        match rotate.dir {
            'L' => {
                while clicks > 0 {
                    start_amount -= 1;
                    if start_amount < 0 {
                        start_amount = 99;
                    }
                    if start_amount == 0 {
                        password += 1;
                    }
                    clicks -= 1;
                }
            }
            'R' => {
                while clicks > 0 {
                    start_amount += 1;
                    if start_amount > 99 {
                        start_amount = 0;
                    }
                    if start_amount == 0 {
                        password += 1;
                    }
                    clicks -= 1;
                }
            }
            _ => panic!("Unknown direction"),
        }
    }

    password
}

mod test {
    use crate::p_01::{get_password, get_password_0x434C49434B};

    #[test]
    fn test_basic() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_01_small.txt";
        println!("the password is : {}", get_password(PATH));
    }

    #[test]
    fn test_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p01_1.txt";
        println!("the password is : {}", get_password(PATH));
    }

    #[test]
    fn test_basic_0x434C49434B() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_01_small.txt";
        println!("the password is : {}", get_password_0x434C49434B(PATH));
    }

    #[test]
    fn test_1_0x434C49434B() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p01_1.txt";
        println!("the password is : {}", get_password_0x434C49434B(PATH));
    }
}
