mod counting;
mod display;
mod fastq;

use crate::counting::Counter;
use crate::display::print_as_table;
use crate::fastq::SequenceReader;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file to process
    filename: String,
}

fn main() {
    let args = Args::parse();

    let mut counter = Counter::new(
        vec!["AA", "GA", "TA", "CAG", "CAA", "CGG", "GAG", "GATA"],
        5,
    );

    let reader = SequenceReader::new(&args.filename);
    for sequence in reader {
        counter.add(&sequence);
    }

    let result = counter.get_results();

    print_as_table(result);
}
