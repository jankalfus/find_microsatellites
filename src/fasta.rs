use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_all_lines<P>(filename: P) -> io::Result<String>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(BufReader::new(file)
        .lines()
        .skip(1)
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join(""))
}

pub struct FastaSequenceReader<'a> {
    filename: &'a str,
}

impl<'a> FastaSequenceReader<'a> {
    pub fn new(filename: &'a str) -> FastaSequenceReader {
        FastaSequenceReader {
            filename,
        }
    }

    pub fn read(& self) -> String {
        read_all_lines(self.filename).unwrap()
    }
}
