// src/main.rs
/*
 * Main executable for ClimateForecaster
 */

use clap::Parser;
use climateforecaster::{Result, run};

#[derive(Parser)]
#[command(version, about = "ClimateForecaster - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
