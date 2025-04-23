mod runner;
mod statistic;

use runner::*;
use statistic::linecount::LineCount;
use statistic::qualfilter::QualityFilterStat;
use statistic::regfilter::RegionFilterStat; 

use clap::Parser;
use std::io::{self};

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    bam_file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut runner = WorkflowRunner::new();

    // Add customization as inputs instead of manually editing here (expand the struct?)
    runner.add_statistic(Box::new(RegionFilterStat::new("2".to_string(), 1, 3)));

    runner.add_statistic(Box::new(QualityFilterStat::new(20.0)));

    runner.add_statistic(Box::new(LineCount::new()));

    runner.process(&args.bam_file)?;
    //Add more features. Min overlap, read depth, etc.
    let finalized_stats = runner.finalize();
    for stat in finalized_stats {
        println!("{}", stat.finalize());
    }

    Ok(())
}

//cargo run -- -b data/readsbam.bam
//cargo build
//./target/debug/bam_workflow_runner -b data/multireadsbam.bam