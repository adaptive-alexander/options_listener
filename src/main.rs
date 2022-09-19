// #![windows_subsystem = "windows"]

use chrono::offset::Utc;
use options::options_struct::Options;
use options::pricing_models::black_scholes;
use options::utilities;
use processing_listener::dir_listener;
use rayon::prelude::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, thread};

/// # run_opt_calc
/// Run function for listener
///
/// # args:
/// *`inp_path` - Path for input file.
/// *`out_path` - Path for output file.
/// *`move_path` - Path to move processed input files into.
fn run_opt_calc(inp_path: PathBuf, out_path: PathBuf, move_path: PathBuf) {
    let from_move_path = PathBuf::from(&inp_path);

    // Initialize data
    thread::sleep(Duration::from_millis(1)); // Avoids OS sometimes still using file when moved into "input"

    // Timing initialization
    let mut start = Instant::now();
    println!("Started processing options at: {}", Utc::now());
    let mut opts = Options::from_file(&inp_path, Box::new(black_scholes::BlackScholesModel::new()));
    println!(
        "Time to parse inputs: {} ms (OS bound)",
        start.elapsed().as_millis()
    );

    // Timing computation and chunking
    start = Instant::now();

    // Chunk options
    let mut chunked_opts = utilities::chunk_opt(opts, 1000);

    // Timing computation
    let start_comp = Instant::now();
    // Parallel computation of options_old
    chunked_opts.par_iter_mut().for_each(|x| x.get_prices());
    chunked_opts.par_iter_mut().for_each(|x| x.get_greeks());
    println!(
        "Time for computation: {} µs",
        start_comp.elapsed().as_micros()
    );

    // Collect Options
    opts = utilities::collect_chunks(chunked_opts);
    println!(
        "Time to compute including chunking and collecting: {} µs",
        start.elapsed().as_micros()
    );

    // Write and time output
    start = Instant::now();
    opts.write_csv(out_path)
        .expect("Failed writing output to csv.");
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
    // todo!("Feature: Add support to parse args for either dir or pubsub")
    dir_listener::dir_listener(
        &run_opt_calc,
        &100, // Sleep in ms
        r".\input",
        r".\output",
        r".\input\processed",
    )
    .await;
}
