use std::fs::read_to_string;

fn load_file(filename: &str) -> Vec<String> {
    let contents = read_to_string(filename).expect("something went wrong reading the file");
    let lines = contents.lines();
    // We need to_owned here because we need to actually put some data on the heap!
    // Originally I was returning a Vec<&str> but my values
    // did not have a long enough lifetime. Really cool!
    lines.map(|s| s.to_owned()).collect::<Vec<String>>()
}

fn convert(num_str: &str) -> Result<i32, std::num::ParseIntError> {
    // .parse works when a leading + or - is included, e.g.,
    // "+23".parse() or "-7".parse()
    num_str.parse::<i32>()
}

// Use generics to accept anything that can give us an iterator
// of anything that can be converted to a str
fn count<I: IntoIterator<Item = T>, T: AsRef<str>>(input: I) -> i32 {
    input
        .into_iter()
        .map(|x| match convert(x.as_ref()) {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Bad value; substituting 0. {}", err);
                0
            }
        }).sum::<i32>()
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
    let result = count(&problem);
    //let result2 = count(&["1".to_string()]);
    println!("Frequency is {}", result);
    result
}

mod examples {
    pub static EXAMPLE1: [&'static str; 3] = ["+1", "+1", "+1"];
    pub static EXAMPLE2: [&'static str; 3] = ["+1", "+1", "-2"];
    pub static EXAMPLE3: [&'static str; 3] = ["-1", "-2", "-3"];
    pub static EXAMPLE4: [&'static str; 3] = ["1", "HELLO THERE", "1"];
}

#[cfg(test)]
mod tests {
    use super::count;
    use super::examples::*;

    #[test]
    fn all_positive() {
        let problem = ["+1", "+1", "+1"];
        let result = count(&problem);
        assert_eq!(result, 3);
    }

    #[test]
    fn pos_and_neg() {
        let problem = EXAMPLE2.to_vec();
        let result = count(&problem);
        assert_eq!(result, 0);
    }
    #[test]
    fn all_neg() {
        let problem = EXAMPLE3.to_vec();
        let result = count(&problem);
        assert_eq!(result, -6);
    }

    #[test]
    fn bad_input() {
        let problem = EXAMPLE4.to_vec();
        let result = count(&problem);
        assert_eq!(result, 2);
    }
}
