mod counting;
mod fastq;

use crate::counting::Counter;
use crate::fastq::SequenceReader;

use clap::Parser;
use itertools::Itertools;
use tabled::{Style, Table, Tabled};

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

    let res = counter.get_results();

    let results: Vec<Result> = res
        .occurrences
        .iter()
        .sorted_by(|&a, &b| a.1.cmp(b.1))
        .rev()
        .map(|(&nuc, &count)| Result {
            nucleotide: nuc.to_owned(),
            count,
            total_length: count * nuc.len() as u64,
            total_percent: count as f64 * nuc.len() as f64 / res.total_reads_length as f64
                * 100_f64,
        })
        .collect();

    let table = Table::new(results).with(Style::modern()).to_string();

    println!("Total reads length: {}", res.total_reads_length);
    println!("{}", table);
}

#[derive(Tabled)]
struct Result {
    #[tabled(rename = "Nucleotide")]
    nucleotide: String,

    #[tabled(rename = "Count")]
    count: u64,

    #[tabled(rename = "Total length")]
    total_length: u64,

    #[tabled(rename = "% of total reads length")]
    total_percent: f64,
}
