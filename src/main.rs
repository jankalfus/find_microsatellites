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

    let nucleotides = vec![
        "TT", "TC", "TG", "AC", "AG", "CC", "AA", "GA", "CA",
        "GT", "CT", "GG", "CG", "AT", "TTA", "TTC", "TTG", "TAT",
        "TAA", "TAC","TAG", "TCT", "TCA", "TCC", "TCG", "TGT", "TGA", "TGC", "TGG","ATT", "ATC", "ATG", "AAC", "AAG", "ACT",
        "ACA", "ACC", "CGT", "AGT", "AGA", "AGC", "AGG", "CTC", "CAC", "CAG", "CCG", "CGC", "GTC", "GCC",
        "TAA", "GAA", "CAA", "ATA", "TTA",
        "GTA", "CTA", "AGA", "TGA", "GGA", "CGA", "ACA", "TCA", "GCA", "CCA", "AAT", "GAT",
        "CAT", "GTT", "CTT", "ATG", "TGT", "GGT", "ACG", "ACT", "TCT", "GCT", "CCT", "GAG",
        "GTG", "CTG", "CGG", "GCG", "GAC", "GGC", "AAAC", "AAAT", "AACC", "AACG", "AAGC",
        "AAGG", "ACAT", "AGAT", "AGCC", "ATAG", "ATCC", "ATGC", "ATGG", "ATTC", "CAAC",
        "CATC", "CCAA", "CTGG", "GAAA", "GAGG", "GATA", "GTAG", "TACA", "TAGA", "TCTG",
        "TGGT", "TATG", "GTTT", "ATTT", "GGTT", "CGTT", "GCTT", "CCTT", "ATGT", "ATCT",
        "GGCT", "CTAT", "GGAT", "GCAT", "CCAT", "GAAT", "GTTG", "GATG", "TTGG", "CCAG", "TTTC",
        "CCTC", "TATC", "CTAC", "TGTA", "TCTA", "CAGA", "ACCA", "CATA", "AATT", "CTAG", "TGCA", "GATTC", "AGGTG", "TCGCC",
        "AGCTG", "CACTT", "GTTTT", "CCTAAC", "CCGGTA", "AGCGGG", "CAAAAA", "GCCTGGT", "GAATC", "CACCT", "GGCGA", "CAGCT",
        "AAGTG", "AAAAC", "GTTAGG", "TACCGG", "CCCGCT", "TTTTTG", "ACCAGGC"
    ];

    let mut counter = Counter::new(
        &nucleotides,
        5,
    );

    let reader = SequenceReader::new(&args.filename);
    for sequence in reader {
        counter.add(&sequence);
    }

    let result = counter.get_results();

    println!("{}", result.total_reads_length);

    for nucleotide in nucleotides {
        let maybe_count = result.occurrences.get(nucleotide);
        if let Some(count) = maybe_count {
            println!("{nucleotide};{count};{};", count * nucleotide.len() as u64);
        }
    }
    // print_as_table(result);
}
