use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn read_input(path: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path).expect("Unable to read file").lines() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        grid.push(row);
    }
    grid
}

fn part_1(path: &str) -> usize {
    let grid = read_input(path);

    let mut obstacles: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut beam_cols: HashSet<usize> = HashSet::new();
    let mut beam_row = 0;

    // get the starting location
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                beam_row = i;
                beam_cols.insert(j);
            } else if *ch == '^' {
                obstacles
                    .entry(i)
                    .and_modify(|vals| {
                        vals.insert(j);
                    })
                    .or_insert_with(|| {
                        let mut hs = HashSet::new();
                        hs.insert(j);
                        hs
                    });
            }
        }
    }

    let mut number_splits = 0;

    while beam_row < grid.len() - 1 {
        if !obstacles.contains_key(&(beam_row + 1)) {
            beam_row += 1;
            continue;
        }
        let next_row_obstacles = obstacles.get(&(beam_row + 1)).unwrap();
        let mut new_beam_cols: HashSet<usize> = HashSet::new();
        for col in beam_cols.iter() {
            if next_row_obstacles.contains(col) {
                number_splits += 1;
                if (*col as i32) - 1 >= 0 {
                    new_beam_cols.insert(col - 1);
                }
                if col + 1 < grid[0].len() {
                    new_beam_cols.insert(col + 1);
                }
            } else {
                new_beam_cols.insert(*col);
            }
        }
        beam_cols = new_beam_cols;
        beam_row += 1;
    }

    number_splits
}

fn part_2(path: &str) -> usize {
    let grid = read_input(path);

    let mut obstacles: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut beam_cols: HashSet<usize> = HashSet::new();
    let mut beam_row = 0;

    let mut all_beam_splits: HashMap<usize, usize> = HashMap::new();

    // get the starting location
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                beam_row = i;
                beam_cols.insert(j);
                all_beam_splits.insert(j, 1);
            } else if *ch == '^' {
                obstacles
                    .entry(i)
                    .and_modify(|vals| {
                        vals.insert(j);
                    })
                    .or_insert_with(|| {
                        let mut hs = HashSet::new();
                        hs.insert(j);
                        hs
                    });
            }
        }
    }

    let mut number_splits = 0;

    while beam_row < grid.len() - 1 {
        if !obstacles.contains_key(&(beam_row + 1)) {
            beam_row += 1;
            continue;
        }
        let next_row_obstacles = obstacles.get(&(beam_row + 1)).unwrap();
        let mut new_beam_cols: HashSet<usize> = HashSet::new();
        for col in beam_cols.iter() {
            if next_row_obstacles.contains(col) {
                number_splits += 1;
                let count_col = all_beam_splits.remove(&col);
                if (*col as i32) - 1 >= 0 {
                    new_beam_cols.insert(col - 1);
                    all_beam_splits
                        .entry(col - 1)
                        .and_modify(|v| *v += count_col.unwrap())
                        .or_insert(count_col.unwrap());
                }
                if col + 1 < grid[0].len() {
                    new_beam_cols.insert(col + 1);
                    all_beam_splits
                        .entry(col + 1)
                        .and_modify(|v| *v += count_col.unwrap())
                        .or_insert(count_col.unwrap());
                }
            } else {
                new_beam_cols.insert(*col);
            }
        }
        beam_cols = new_beam_cols;
        beam_row += 1;
    }
    let mut counts = 0;
    for (_, v) in all_beam_splits.iter() {
        counts += *v;
    }
    counts
}

#[cfg(test)]
mod test {
    use crate::p_07::{part_1, part_2, read_input};

    #[test]
    fn test_sample_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_07_sample.txt";
        println!("The solution is : {}", part_1(PATH));
    }

    #[test]
    fn test_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_07.txt";
        println!("The solution is : {}", part_1(PATH));
    }

    #[test]
    fn test_sample_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_07_sample.txt";
        println!("The solution is : {}", part_2(PATH));
    }

    #[test]
    fn test_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_07.txt";
        println!("The solution is : {}", part_2(PATH));
    }
}
