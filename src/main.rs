// #![windows_subsystem = "windows"]

use chrono::offset::Utc;
use chrono::DateTime;
use options::*;
use processing_listener::listener;
use rayon::prelude::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, thread};

fn run_opt_calc(inp_path: PathBuf, out_path: PathBuf, move_path: PathBuf) {
    let from_move_path = PathBuf::from(&inp_path);

    // Initialize data
    thread::sleep(Duration::from_millis(1)); // Avoids OS sometimes still using file when moved into "input"

    // Timing initialization
    let mut start = Instant::now();
    println!("Started processing options at: {}", Utc::now());
    let data = parse_input::parse_input(inp_path);
    println!(
        "Time to parse inputs: {} ms (OS bound)",
        start.elapsed().as_millis()
    );

    // Timing computation
    start = Instant::now();

    // Chunk data
    let chunk_data: (
        Vec<Vec<String>>,
        Vec<Vec<OptTypes>>,
        Vec<Vec<f64>>,
        Vec<Vec<f64>>,
        Vec<Vec<DateTime<Utc>>>,
        Vec<Vec<DateTime<Utc>>>,
        Vec<Vec<f64>>,
        Vec<Vec<f64>>,
        Vec<Vec<f64>>,
    ) = (
        data.0.into_par_iter().chunks(1000).collect(),
        data.1.into_par_iter().chunks(1000).collect(),
        data.2.into_par_iter().chunks(1000).collect(),
        data.3.into_par_iter().chunks(1000).collect(),
        data.4.into_par_iter().chunks(1000).collect(),
        data.5.into_par_iter().chunks(1000).collect(),
        data.6.into_par_iter().chunks(1000).collect(),
        data.7.into_par_iter().chunks(1000).collect(),
        data.8.into_par_iter().chunks(1000).collect(),
    );

    // Parallel computation of options
    let opts: Vec<Options> = chunk_data.into_par_iter().map(initialize_opts).collect();
    println!("Time to compute: {} µs", start.elapsed().as_micros());

    // Write and time output
    start = Instant::now();
    write_csv_out(out_path, opts).expect("Failed writing output to csv.");
    println!(
        "Time to write result: {} ms (OS bound)",
        start.elapsed().as_millis()
    );

    // Move input file
    fs::rename(from_move_path, move_path).expect("Failed to rename file.");
    println!("Moved input file to processed path.");
}

#[tokio::main]
async fn main() {
    listener(
        &run_opt_calc,
        &100, // Sleep in ms
        r".\input",
        r".\output",
        r".\input\processed",
    )
    .await;
}
