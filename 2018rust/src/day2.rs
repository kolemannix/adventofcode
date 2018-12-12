use crate::file::load_file;
use std::collections::HashMap;

fn has_two_or_three(input: &str) -> (bool, bool) {
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
    let mut has_two = false;
    let mut has_three = false;
    result_map.iter().for_each(|(_, v)| {
        if *v == 2 as usize {
            has_two = true;
        }
        if *v == (3 as usize) {
            has_three = true;
        }
    });
    (has_two, has_three)
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

pub fn part1() -> i32 {
    let problem = load_file("src/day2/day2input.txt");
    let solution = checksum(problem);
    solution
}

pub fn part2() -> String {
    let input = load_file("src/day2/day2input.txt");
    let mut ans = None;
    for v in &input {
        for p in &input {
            let mut diff = 0;
            let mut mismatch_idx = None;
            let v_chars = v.chars().enumerate();
            let mut p_chars = p.chars();
            for (i, c) in v_chars {
                if Some(c) != p_chars.next() {
                    mismatch_idx = Some(i);
                    diff = diff + 1;
                }
            }
            if diff == 1 {
                let mut answer = v.clone();
                answer.remove(mismatch_idx.expect("No mismatch_idx"));
                ans = Some(answer);
            }
        }
    }
    match ans {
        Some(a) => a,
        None => panic!("No answer!!!"),
    }
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
