use std::collections::HashMap;

fn has_two_or_three(input: &str) -> (bool, bool) {
    let mut result_map: HashMap<char, usize> = HashMap::new();
    let mut exactly_two = (None, false);
    let mut exactly_three = (None, false);
    for c in input.chars() {
        match result_map.get_mut(&c) {
            Some(count) => {
                *count = *count + 1;
                if *count == 2 {
                    exactly_two = (Some(c), true);
                }
                if *count == 3 {
                    if exactly_two.0 == Some(c) {
                        exactly_two = (None, false);
                    }
                    exactly_three = (Some(c), true);
                }
            }
            None => {
                result_map.insert(c, 1);
            }
        }
    }
    (exactly_two.1, exactly_three.1)
}

fn count_frequencies(input: &str) -> HashMap<char, usize> {
    let mut result_map: HashMap<char, usize> = HashMap::new();

    for c in input.chars() {
        match result_map.get_mut(&c) {
            Some(count) => {
                *count = *count + 1;
            }
            None => {
                result_map.insert(c, 1);
            }
        }
    }
    result_map
}

fn checksum<T: AsRef<str>>(input: Vec<T>) -> i32 {
    let init: (i32, i32) = (0, 0);
    let (twos, threes) = input
        .iter()
        .fold(init, |acc, x| match has_two_or_three(x.as_ref()) {
            (true, true) => (acc.0 + 1, acc.1 + 1),
            (true, false) => (acc.0 + 1, acc.1),
            (false, true) => (acc.0, acc.1 + 1),
            (false, false) => acc,
        });
    twos * threes
}

pub fn solve() -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use super::checksum;

    #[test]
    fn test1() {
        let problem = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(checksum(problem), 12)
    }
}
