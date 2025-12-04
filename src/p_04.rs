use std::fs::read_to_string;

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn read_input(path: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path).expect("Unable to read input").lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

fn part1(path: &str) -> u64 {
    let grid = read_input(path);
    part1_helper(&grid)
}

fn part1_helper(grid: &Vec<Vec<char>>) -> u64 {
    let n = grid.len();
    let m = grid[0].len();
    let mut accessible_loc: u64 = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '@' {
                continue;
            }
            let n_rolls = get_neighbour_rolls(&grid, i, j);
            if n_rolls < 4 {
                accessible_loc += 1;
            }
        }
    }
    accessible_loc
}

fn get_neighbour_rolls(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    let mut n_count: u64 = 0;
    for (dx, dy) in DIRS {
        if (x as i32 + dx) >= 0
            && (y as i32 + dy) >= 0
            && (x as i32 + dx) < grid.len() as i32
            && (y as i32 + dy) < grid[0].len() as i32
        {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            if grid[new_x][new_y] == '@' {
                n_count += 1;
            }
        }
    }
    n_count
}

struct CellState {
    val: char,
    n_rolls: u64,
}

fn part2(path: &str) -> u64 {
    let mut grid = read_input(path);
    loop {
        let (enhanced_grid, num_accessible) = enhance_grid(&grid);
        if num_accessible == 0 {
            break;
        }
        grid = modify_grid(grid, enhanced_grid);
    }
    part2_helper(&grid, path)
}

fn part2_helper(grid: &Vec<Vec<char>>, path: &str) -> u64 {
    let mut accessible_count = 0;
    let original_grid = read_input(path);
    for i in 0..original_grid.len() {
        for j in 0..original_grid[0].len() {
            if original_grid[i][j] == '@' && grid[i][j] == '.' {
                accessible_count += 1;
            }
        }
    }
    accessible_count
}

fn modify_grid(mut grid: Vec<Vec<char>>, enhanced_grid: Vec<Vec<CellState>>) -> Vec<Vec<char>> {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if enhanced_grid[i][j].val == '@' && enhanced_grid[i][j].n_rolls < 4 {
                grid[i][j] = '.'
            }
        }
    }
    grid
}

fn enhance_grid(grid: &Vec<Vec<char>>) -> (Vec<Vec<CellState>>, u64) {
    let mut num_accessible: u64 = 0;
    let mut enhanced_grid: Vec<Vec<CellState>> = Vec::new();
    for i in 0..grid.len() {
        let mut enhanced_row: Vec<CellState> = Vec::new();
        for j in 0..grid[0].len() {
            let cstate = enhance_helper(grid, i, j);
            if cstate.val == '@' && cstate.n_rolls < 4 {
                num_accessible += 1;
            }
            enhanced_row.push(cstate);
        }
        enhanced_grid.push(enhanced_row);
    }
    (enhanced_grid, num_accessible)
}

fn enhance_helper(grid: &Vec<Vec<char>>, x: usize, y: usize) -> CellState {
    let mut n_count: u64 = 0;
    for (dx, dy) in DIRS {
        if (x as i32 + dx) >= 0
            && (y as i32 + dy) >= 0
            && (x as i32 + dx) < grid.len() as i32
            && (y as i32 + dy) < grid[0].len() as i32
        {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            if grid[new_x][new_y] == '@' {
                n_count += 1;
            }
        }
    }
    CellState {
        val: grid[x][y],
        n_rolls: n_count,
    }
}

fn print_helper(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

mod test {
    use crate::p_04::{part1, part2, read_input};

    #[test]
    fn test_sample_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_04_sample.txt";
        println!("The solution is {}", part1(PATH));
    }

    #[test]
    fn test_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_04.txt";
        println!("The solution is {}", part1(PATH));
    }

    #[test]
    fn test_sample_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_04_sample.txt";
        println!("The solution is {}", part2(PATH));
    }

    #[test]
    fn test_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_04.txt";
        println!("The solution is {}", part2(PATH));
    }
}
