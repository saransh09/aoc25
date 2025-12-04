// I asked AI to generate a cute visualisation for the problem 4

use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, Rgb, RgbImage};
use std::fs::{File, read_to_string};

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

// Color palette
const COLOR_EMPTY: Rgb<u8> = Rgb([10, 10, 10]); // Near black
const COLOR_ROCK_SAFE: Rgb<u8> = Rgb([68, 255, 68]); // Bright green
const COLOR_ROCK_DANGER: Rgb<u8> = Rgb([255, 170, 0]); // Orange
const COLOR_ROCK_DYING: Rgb<u8> = Rgb([255, 68, 68]); // Red
const COLOR_GRID: Rgb<u8> = Rgb([50, 50, 50]); // Dark gray

type Grid = Vec<Vec<char>>;

#[derive(Clone)]
struct GridState {
    grid: Grid,
    neighbor_counts: Vec<Vec<u8>>,
    cells_to_remove: Vec<(usize, usize)>,
    iteration: usize,
}

fn read_input(path: &str) -> Grid {
    let mut grid: Grid = Vec::new();
    for line in read_to_string(path).expect("Unable to read input").lines() {
        if line.trim().is_empty() {
            continue;
        }
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    grid
}

fn get_neighbor_count(grid: &Grid, x: usize, y: usize) -> u8 {
    let mut count = 0u8;
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;

    for (dx, dy) in DIRS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && ny >= 0 && nx < n && ny < m {
            if grid[nx as usize][ny as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

fn enhance_grid(grid: &Grid) -> (Vec<Vec<u8>>, Vec<(usize, usize)>) {
    let n = grid.len();
    let m = grid[0].len();
    let mut neighbor_counts = vec![vec![0u8; m]; n];
    let mut cells_to_remove = Vec::new();

    for i in 0..n {
        for j in 0..m {
            let count = get_neighbor_count(grid, i, j);
            neighbor_counts[i][j] = count;
            if grid[i][j] == '@' && count < 4 {
                cells_to_remove.push((i, j));
            }
        }
    }

    (neighbor_counts, cells_to_remove)
}

fn modify_grid(mut grid: Grid, cells_to_remove: &[(usize, usize)]) -> Grid {
    for &(x, y) in cells_to_remove {
        grid[x][y] = '.';
    }
    grid
}

fn simulate_evolution(input_path: &str, max_iterations: usize) -> Vec<GridState> {
    let mut grid = read_input(input_path);
    let mut states = Vec::new();

    println!(
        "Starting simulation on {}x{} grid...",
        grid.len(),
        grid[0].len()
    );

    for iteration in 0..max_iterations {
        if iteration % 10 == 0 {
            println!("  Iteration {}...", iteration);
        }

        let (neighbor_counts, cells_to_remove) = enhance_grid(&grid);

        states.push(GridState {
            grid: grid.clone(),
            neighbor_counts,
            cells_to_remove: cells_to_remove.clone(),
            iteration,
        });

        if cells_to_remove.is_empty() {
            println!("Converged at iteration {}", iteration);
            break;
        }

        grid = modify_grid(grid, &cells_to_remove);
    }

    states
}

fn get_cell_color(state: &GridState, x: usize, y: usize) -> Rgb<u8> {
    let cell = state.grid[x][y];

    if cell == '.' {
        return COLOR_EMPTY;
    }

    if cell == '@' {
        let count = state.neighbor_counts[x][y];
        if count < 2 {
            COLOR_ROCK_DYING // Very vulnerable
        } else if count < 4 {
            COLOR_ROCK_DANGER // Will be removed
        } else {
            COLOR_ROCK_SAFE // Safe
        }
    } else {
        COLOR_EMPTY
    }
}

fn create_frame(state: &GridState, cell_size: u32) -> RgbImage {
    let n = state.grid.len() as u32;
    let m = state.grid[0].len() as u32;
    let width = m * cell_size;
    let height = n * cell_size;

    let mut img = ImageBuffer::new(width, height);

    // Fill background
    for pixel in img.pixels_mut() {
        *pixel = COLOR_EMPTY;
    }

    // Draw cells
    for i in 0..n as usize {
        for j in 0..m as usize {
            let color = get_cell_color(state, i, j);

            // Fill cell
            for dy in 0..cell_size {
                for dx in 0..cell_size {
                    let px = j as u32 * cell_size + dx;
                    let py = i as u32 * cell_size + dy;

                    // Draw grid lines
                    if dx == 0 || dy == 0 {
                        if cell_size > 2 {
                            img.put_pixel(px, py, COLOR_GRID);
                        } else {
                            img.put_pixel(px, py, color);
                        }
                    } else {
                        img.put_pixel(px, py, color);
                    }
                }
            }
        }
    }

    img
}

fn save_frame_as_png(
    state: &GridState,
    cell_size: u32,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = create_frame(state, cell_size);
    img.save(path)?;
    println!("Saved frame to {}", path);
    Ok(())
}

fn create_gif_animation(
    states: &[GridState],
    cell_size: u32,
    output_path: &str,
    frame_delay: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let n = states[0].grid.len() as u32;
    let m = states[0].grid[0].len() as u32;
    let width = m * cell_size;
    let height = n * cell_size;

    println!("Creating GIF animation with {} frames...", states.len());
    println!(
        "Frame size: {}x{} ({}x{} cells at {} pixels each)",
        width, height, m, n, cell_size
    );

    // Create a color palette for the GIF
    let mut palette = Vec::new();
    palette.extend_from_slice(&COLOR_EMPTY.0); // Index 0
    palette.extend_from_slice(&COLOR_ROCK_SAFE.0); // Index 1
    palette.extend_from_slice(&COLOR_ROCK_DANGER.0); // Index 2
    palette.extend_from_slice(&COLOR_ROCK_DYING.0); // Index 3
    palette.extend_from_slice(&COLOR_GRID.0); // Index 4
    palette.extend_from_slice(&[26, 26, 26]); // Index 5 (dark background)

    // Pad palette to 256 colors (GIF requirement)
    while palette.len() < 256 * 3 {
        palette.extend_from_slice(&[0, 0, 0]);
    }

    let file = File::create(output_path)?;
    let mut encoder = Encoder::new(file, width as u16, height as u16, &palette)?;
    encoder.set_repeat(Repeat::Infinite)?;

    for (idx, state) in states.iter().enumerate() {
        if idx % 5 == 0 {
            println!("  Encoding frame {}/{}...", idx + 1, states.len());
        }

        let img = create_frame(state, cell_size);

        // Convert RGB image to indexed color using our palette
        let indexed_pixels: Vec<u8> = img
            .pixels()
            .map(|pixel| {
                // Map pixel color to palette index
                let rgb = [pixel[0], pixel[1], pixel[2]];
                if rgb == COLOR_EMPTY.0 {
                    0
                } else if rgb == COLOR_ROCK_SAFE.0 {
                    1
                } else if rgb == COLOR_ROCK_DANGER.0 {
                    2
                } else if rgb == COLOR_ROCK_DYING.0 {
                    3
                } else if rgb == COLOR_GRID.0 {
                    4
                } else {
                    0
                } // default to empty
            })
            .collect();

        let mut frame = Frame::default();
        frame.width = width as u16;
        frame.height = height as u16;
        frame.delay = frame_delay;
        frame.buffer = std::borrow::Cow::Owned(indexed_pixels);

        encoder.write_frame(&frame)?;
    }

    println!("GIF animation saved to {}", output_path);
    Ok(())
}

fn create_summary_image(
    states: &[GridState],
    cell_size: u32,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let num_states = states.len();
    let num_frames = 6.min(num_states);

    // Select frames to display
    let selected_indices: Vec<usize> = if num_states <= 6 {
        (0..num_states).collect()
    } else {
        vec![
            0,
            num_states / 4,
            num_states / 2,
            3 * num_states / 4,
            num_states - 2,
            num_states - 1,
        ]
    };

    let frame_width = states[0].grid[0].len() as u32 * cell_size;
    let frame_height = states[0].grid.len() as u32 * cell_size;
    let padding = 10u32;

    let cols = 3u32;
    let rows = (num_frames as u32 + cols - 1) / cols;

    let total_width = cols * frame_width + (cols + 1) * padding;
    let total_height = rows * frame_height + (rows + 1) * padding;

    let mut summary_img = ImageBuffer::new(total_width, total_height);

    // Fill background with dark color
    for pixel in summary_img.pixels_mut() {
        *pixel = Rgb([26, 26, 26]);
    }

    println!(
        "Creating summary image with {} key frames...",
        selected_indices.len()
    );

    for (idx, &state_idx) in selected_indices.iter().enumerate() {
        let row = idx as u32 / cols;
        let col = idx as u32 % cols;

        let x_offset = (col + 1) * padding + col * frame_width;
        let y_offset = (row + 1) * padding + row * frame_height;

        let frame = create_frame(&states[state_idx], cell_size);

        // Copy frame to summary image
        for y in 0..frame_height {
            for x in 0..frame_width {
                let pixel = frame.get_pixel(x, y);
                summary_img.put_pixel(x_offset + x, y_offset + y, *pixel);
            }
        }
    }

    summary_img.save(output_path)?;
    println!("Summary image saved to {}", output_path);
    Ok(())
}

fn print_stats(states: &[GridState]) {
    println!("\n{}", "=".repeat(60));
    println!("EVOLUTION STATISTICS");
    println!("{}", "=".repeat(60));
    println!("Total iterations: {}", states.len());
    println!(
        "Grid size: {}x{}",
        states[0].grid.len(),
        states[0].grid[0].len()
    );

    let initial_rocks: usize = states[0]
        .grid
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '@').count())
        .sum();

    let final_rocks: usize = states
        .last()
        .unwrap()
        .grid
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '@').count())
        .sum();

    println!("Initial rock cells: {}", initial_rocks);
    println!("Final rock cells: {}", final_rocks);
    println!("Cells made accessible: {}", initial_rocks - final_rocks);

    println!("\nFirst 10 iterations:");
    for state in states.iter().take(10) {
        let rock_count: usize = state
            .grid
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '@').count())
            .sum();
        println!(
            "  Iteration {}: {} rocks, {} will be removed",
            state.iteration,
            rock_count,
            state.cells_to_remove.len()
        );
    }

    if states.len() > 10 {
        println!("  ... ({} more iterations)", states.len() - 10);
    }

    println!("{}", "=".repeat(60));
}

pub fn visualize(
    input_path: &str,
    output_gif: Option<&str>,
    output_summary: Option<&str>,
    cell_size: u32,
    frame_delay: u16,
    max_iterations: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let states = simulate_evolution(input_path, max_iterations);
    print_stats(&states);

    if let Some(gif_path) = output_gif {
        create_gif_animation(&states, cell_size, gif_path, frame_delay)?;
    }

    if let Some(summary_path) = output_summary {
        create_summary_image(&states, cell_size, summary_path)?;
    }

    println!("\n✓ Visualization complete!");
    Ok(())
}

// Example usage in tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualize_sample() {
        visualize(
            "src/p_04_sample.txt",
            Some("rust_sample_evolution.gif"),
            Some("rust_sample_summary.png"),
            20, // 20 pixels per cell
            80, // 800ms delay (80 * 10ms)
            100,
        )
        .unwrap();
    }

    #[test]
    fn test_visualize_full() {
        visualize(
            "src/p_04.txt",
            Some("rust_full_evolution.gif"),
            Some("rust_full_summary.png"),
            4,  // 4 pixels per cell (large grid)
            50, // 500ms delay
            1000,
        )
        .unwrap();
    }
}
