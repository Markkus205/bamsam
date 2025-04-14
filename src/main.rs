mod runner;
mod statistic;

use runner::*;
use statistic::linecount::LineCount;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader};

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)] //we can remove this dont need a struct
    bam_file: String,
}

fn main() -> io::Result<()> {

    let args = Args::parse();


    let bam_file = File::open(&args.bam_file)?;
    let reader = BufReader::new(bam_file);


    let mut runner = WorkflowRunner::new();
    runner.add_statistic(Box::new(LineCount::new()));


    runner.process(reader)?;

    let finalized_stats = runner.finalize();
    for stat in finalized_stats {
        println!("{}", stat.finalize());
    }

    Ok(())
}
//HOW WOULD I PIPE THESE FUNCTIONS? DOES THAT EVEN WORK WITH A RUNNER?