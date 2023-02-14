use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines())
}

pub struct FastqSequenceReader<'a> {
    filename: &'a str,
    iter: Option<Lines<BufReader<File>>>,
}

impl<'a> FastqSequenceReader<'a> {
    pub fn new(filename: &'a str) -> FastqSequenceReader {
        FastqSequenceReader {
            filename,
            iter: None,
        }
    }
}

impl<'a> Iterator for FastqSequenceReader<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self
            .iter
            .get_or_insert_with(|| read_lines(self.filename).unwrap());

        let mut use_this = false;

        for maybe_line in iter {
            let line = maybe_line.unwrap();

            if line.starts_with('@') {
                use_this = true;
                continue;
            }

            if use_this {
                return Some(line);
            }
        }

        None
    }
}
