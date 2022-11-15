use crate::counting::CounterResult;

use itertools::Itertools;
use tabled::{Style, Table, Tabled};

pub fn print_as_table(result: CounterResult) {
    let occurrences: Vec<Result> = result
        .occurrences
        .iter()
        .sorted_by(|&a, &b| a.1.cmp(b.1))
        .rev()
        .map(|(&nucleotide, &count)| Result {
            nucleotide: nucleotide.to_owned(),
            count,
            total_length: count * nucleotide.len() as u64,
            total_percent: count as f64 * nucleotide.len() as f64
                / result.total_reads_length as f64
                * 100_f64,
        })
        .collect();

    let table = Table::new(occurrences).with(Style::modern()).to_string();

    println!("Total reads length: {}", result.total_reads_length);
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
