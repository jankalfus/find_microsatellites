mod counting;

use crate::counting::count_matches;

use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use tabled::{Style, Table, Tabled};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file to process
    filename: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args = Args::parse();

    let required_number_of_repetitions = 5;

    let mut occurrences = HashMap::new();

    for nucleotide in ["AA", "GA", "TA", "CAG", "CAA", "CGG", "GAG", "GATA"] {
        occurrences.insert(nucleotide, 0_u64);
    }

    let mut total_reads_length: u64 = 0;

    let lines = read_lines(args.filename).unwrap();

    let mut use_this = false;
    for maybe_line in lines {
        let line = maybe_line.unwrap();

        if line.starts_with('@') {
            use_this = true;
            continue;
        }

        if use_this {
            use_this = false;

            total_reads_length += line.len() as u64;

            for (nuc, count) in occurrences.iter_mut() {
                let match_counts = count_matches(&line, nuc);

                *count += match_counts.iter().fold(0, |acc, &c| {
                    if c >= required_number_of_repetitions {
                        acc + c
                    } else {
                        acc
                    }
                }) as u64;
            }
        }
    }

    let results: Vec<Result> = occurrences
        .iter()
        .sorted_by(|&a, &b| a.1.cmp(b.1))
        .rev()
        .map(|(&nuc, &count)| Result {
            nucleotide: nuc.to_owned(),
            count,
            total_length: count * nuc.len() as u64,
            total_percent: count as f64 * nuc.len() as f64 / total_reads_length as f64 * 100_f64,
        })
        .collect();

    let table = Table::new(results).with(Style::modern()).to_string();

    println!("Total reads length: {}", total_reads_length);
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
