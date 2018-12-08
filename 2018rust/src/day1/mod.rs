use std::collections::HashSet;
use file::load_file;

fn convert(num_str: &str) -> Result<i32, std::num::ParseIntError> {
    // .parse works when a leading + or - is included, e.g.,
    // "+23".parse() or "-7".parse()
    num_str.parse::<i32>()
}

// Use generics to accept anything that can give us an iterator
// of anything that can be converted to a str
fn repeating_balance<I: IntoIterator<Item = T>, T: AsRef<str>>(input: I) -> i32 {
    let input_vec = input
        .into_iter()
        .map(|x| convert(x.as_ref()).unwrap())
        .collect::<Vec<i32>>();

    let mut previous_frequencies: HashSet<i32> = HashSet::new();
    let mut i = 0;
    let mut freq = 0;
    return loop {
        previous_frequencies.insert(freq);
        let cur = input_vec[i];
        let next_freq = freq + cur;
        if previous_frequencies.contains(&next_freq) {
            break next_freq;
        }
        freq = next_freq;
        let next_i = if i == input_vec.len() - 1 { 0 } else { i + 1 };
        i = next_i;
    };
}

// Accept any slice of String; this will work with vector and raw slices
// fn count(input: &[String]) -> i32 {
//     input
//         .iter()
//         .map(|x| match convert(x) {
//             Ok(num) => num,
//             Err(err) => {
//                 eprintln!("Bad value; substituting 0. {}", err);
//                 0
//             }
//         }).sum::<i32>()
// }

pub fn solve() -> i32 {
    let problem = load_file("src/day1/input.txt");
    let result = repeating_balance(&problem);
    //let result2 = count(&["1".to_string()]);
    println!("Frequency is {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::repeating_balance;

    #[test]
    fn test1() {
        let problem = ["+1", "-1"];
        let result = repeating_balance(&problem);
        assert_eq!(result, 0);
    }

    #[test]
    fn test2() {
        let problem = ["+3", "+3", "+4", "-2", "-4"];
        let result = repeating_balance(&problem);
        assert_eq!(result, 10);
    }

    #[test]
    fn test3() {
        let problem = ["-6", "+3", "+8", "+5", "-6"];
        let result = repeating_balance(&problem);
        assert_eq!(result, 5);
    }

    #[test]
    fn test4() {
        let problem = ["+7", "+7", "-2", "-7", "-4"];
        let result = repeating_balance(&problem);
        assert_eq!(result, 14);
    }

}
