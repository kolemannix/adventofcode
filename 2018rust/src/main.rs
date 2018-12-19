mod day1;
mod day2;
mod day3;
mod day4;
mod file;

fn main() {
    println!("Welcome to Advent of Rust 2018!");

    // let day1 = day1::solve();
    // println!("Day 1: {}", day1);

    // println!("Day 2 part 1: {}", day2::part1());
    // println!("Day 2 part 2: {}", day2::part2());

    // println!("Day 3 part 1: {}", day3::part1());
    let (part1, part2) = day4::solve();
    println!("Day 4 part 1: {}", part1);
    println!("Day 4 part 2: {}", part2);
}
