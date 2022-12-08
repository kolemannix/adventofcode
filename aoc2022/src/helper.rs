pub fn load_puzzle_to_string(day_number: usize, part_number: usize) -> std::io::Result<String> {
    std::fs::read_to_string(format!("inputs/day_{}_{}.txt", day_number, part_number))
}
