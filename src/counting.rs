#[cfg(test)]
mod count_matches_tests {
    use super::count_matches;

    #[test]
    fn empty_string() {
        assert_eq!(count_matches("", "AA"), vec![]);
    }

    #[test]
    fn no_match() {
        assert_eq!(count_matches("TATT", "AA"), vec![])
    }

    #[test]
    fn single_match() {
        assert_eq!(count_matches("AA", "AA"), vec![1]);
    }

    #[test]
    fn single_match_in_odd_number_of_chars() {
        assert_eq!(count_matches("AAA", "AA"), vec![1]);
    }

    #[test]
    fn multiple_matches() {
        assert_eq!(count_matches("AAAA", "AA"), vec![2]);
    }

    #[test]
    fn multiple_distant_matches() {
        assert_eq!(count_matches("AATTAA", "AA"), vec![1, 1]);
    }

    #[test]
    fn multiple_various_matches() {
        assert_eq!(count_matches("AAAAAATTAAGAA", "AA"), vec![3, 1, 1]);
    }

    #[test]
    fn odd_pattern() {
        assert_eq!(count_matches("TATTATGATGATTAT", "GAT"), vec![2]);
    }
}

pub fn count_matches(str: &str, pattern: &str) -> Vec<usize> {
    let mut counts = Vec::new();

    let mut it = str.match_indices(pattern);

    let mut current = it.next();
    let mut next = it.next();

    loop {
        if let Some(current_value) = current {
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
        } else {
            break;
        }
    }

    counts
}
