use std::collections::HashMap;

#[cfg(test)]
mod count_matches_tests {
    use super::Counter;

    #[test]
    fn empty_string() {
        assert_eq!(Counter::count_matches("", "AA"), vec![]);
    }

    #[test]
    fn no_match() {
        assert_eq!(Counter::count_matches("TATT", "AA"), vec![])
    }

    #[test]
    fn single_match() {
        assert_eq!(Counter::count_matches("AA", "AA"), vec![1]);
    }

    #[test]
    fn single_match_in_odd_number_of_chars() {
        assert_eq!(Counter::count_matches("AAA", "AA"), vec![1]);
    }

    #[test]
    fn multiple_matches() {
        assert_eq!(Counter::count_matches("AAAA", "AA"), vec![2]);
    }

    #[test]
    fn multiple_distant_matches() {
        assert_eq!(Counter::count_matches("AATTAA", "AA"), vec![1, 1]);
    }

    #[test]
    fn multiple_various_matches() {
        assert_eq!(Counter::count_matches("AAAAAATTAAGAA", "AA"), vec![3, 1, 1]);
    }

    #[test]
    fn odd_pattern() {
        assert_eq!(Counter::count_matches("TATTATGATGATTAT", "GAT"), vec![2]);
    }
}

pub struct Counter<'a> {
    occurrences: HashMap<&'a str, u64>,
    required_number_of_repetitions: usize,
    total_reads_length: u64,
}

impl<'a> Counter<'a> {
    pub fn new(nucleotides: Vec<&str>, required_number_of_repetitions: usize) -> Counter {
        let mut occurrences = HashMap::new();

        for nucleotide in nucleotides {
            occurrences.insert(nucleotide, 0_u64);
        }

        Counter {
            occurrences,
            required_number_of_repetitions,
            total_reads_length: 0,
        }
    }

    pub fn add(&mut self, sequence: &str) {
        self.total_reads_length += sequence.len() as u64;

        for (nuc, count) in self.occurrences.iter_mut() {
            let match_counts = Self::count_matches(sequence, nuc);

            *count += match_counts.iter().fold(0, |acc, &c| {
                if c >= self.required_number_of_repetitions {
                    acc + c
                } else {
                    acc
                }
            }) as u64;
        }
    }

    pub fn get_results(&self) -> CounterResult {
        CounterResult {
            occurrences: &self.occurrences,
            total_reads_length: self.total_reads_length,
        }
    }

    fn count_matches(str: &str, pattern: &str) -> Vec<usize> {
        let mut counts = Vec::new();

        let mut it = str.match_indices(pattern);

        let mut current = it.next();
        let mut next = it.next();

        while let Some(current_value) = current {
            let mut count = 1;

            while let Some(next_value) = next {
                if current_value.0 + pattern.len() * count == next_value.0 {
                    count += 1;
                    next = it.next();
                } else {
                    break;
                }
            }

            current = next;
            next = it.next();

            counts.push(count);
        }

        counts
    }
}

pub struct CounterResult<'a> {
    pub occurrences: &'a HashMap<&'a str, u64>,
    pub total_reads_length: u64,
}
