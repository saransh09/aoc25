mod p_01;
mod p_02;
mod p_02_fp;
mod p_03;
mod p_04;
mod p_05;
mod visualizer_p04;

fn main() {
    println!("AOC 2025 - Advent of Code Solutions");

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "visualize" {
        println!("\n=== Running P04 Visualizer ===\n");

        // Default to full input
        let input_path = if args.len() > 2 && args[2] == "sample" {
            "src/p_04_sample.txt"
        } else {
            "src/p_04.txt"
        };

        let cell_size = if input_path.contains("sample") { 20 } else { 4 };

        visualizer_p04::visualize(
            input_path,
            Some("rust_evolution.gif"),
            Some("rust_summary.png"),
            cell_size,
            50, // 500ms delay per frame
            1000,
        )
        .expect("Visualization failed");
    } else {
        println!("\nUsage:");
        println!("  cargo run                    - Display this message");
        println!("  cargo run visualize          - Visualize full input (p_04.txt)");
        println!("  cargo run visualize sample   - Visualize sample input");
        println!("  cargo test                   - Run all tests including visualizer tests");
    }
}
