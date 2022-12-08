use anyhow::Result;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};
use std::collections::VecDeque;

use itertools::Itertools;

use crate::helper;

fn day_1_part_1() -> Result<String> {
    let input = helper::load_puzzle_to_string(1, 1)?;
    let lines = input.lines();
    let mut elf: u64 = 0;
    let mut max: u64 = 0;
    for line in lines {
        if line.is_empty() {
            if elf > max {
                max = elf;
            }
            elf = 0;
        } else {
            let value: u64 = line
                .parse()
                .unwrap_or_else(|_| panic!("Expected integer; got {line}"));
            elf += value;
        }
    }
    if elf > max {
        max = elf;
    }

    Ok(max.to_string())
}

fn day_1_part_2_vec() -> Result<String> {
    let input = helper::load_puzzle_to_string(1, 1)?;
    let lines = input.lines();
    let mut elf: u64 = 0;
    let mut elves: Vec<u64> = vec![];
    for line in lines {
        if line.is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            let value: u64 = line
                .parse()
                .unwrap_or_else(|_| panic!("Expected integer; got {line}"));
            elf += value;
        }
    }
    elves.sort();
    let sum: u64 = elves.iter().rev().take(3).sum();
    Ok(sum.to_string())
}

fn day_1_part_2_heap() -> Result<String> {
    let input = helper::load_puzzle_to_string(1, 1)?;
    let lines = input.lines();
    let mut elf: u64 = 0;
    let mut elf_heap: BinaryHeap<u64> = BinaryHeap::new();
    for line in lines {
        if line.is_empty() {
            elf_heap.push(elf);
            elf = 0;
        } else {
            let value: u64 = line.parse().unwrap();
            elf += value;
        }
    }
    let sum =
        elf_heap.pop().unwrap_or(0) + elf_heap.pop().unwrap_or(0) + elf_heap.pop().unwrap_or(0);
    Ok(sum.to_string())
}

fn day_2() -> Result<String> {
    let input = helper::load_puzzle_to_string(2, 1)?;
    let mut score = 0;
    let lines = input.lines();
    for line in lines {
        let mut chars = line.chars();
        let opp_move = chars.next().unwrap();
        chars.next();
        let my_move = chars.next().unwrap();
        // X lose
        // Y draw
        // Z win
        match (opp_move, my_move) {
            ('A', 'X') => score += 0 + 3,
            ('A', 'Y') => score += 3 + 1,
            ('A', 'Z') => score += 6 + 2,
            ('B', 'X') => score += 0 + 1,
            ('B', 'Y') => score += 3 + 2,
            ('B', 'Z') => score += 6 + 3,
            ('C', 'X') => score += 0 + 2,
            ('C', 'Y') => score += 3 + 3,
            ('C', 'Z') => score += 6 + 1,
            _ => {}
        }
    }
    Ok(score.to_string())
}

fn day_3_priority(c: char) -> u32 {
    let num_val = match c.to_lowercase().next().unwrap() {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => 0,
    };
    if c.is_uppercase() {
        num_val + 26
    } else {
        num_val
    }
}

fn day_3() -> Result<String> {
    let mut sum = 0;
    let input = helper::load_puzzle_to_string(3, 1)?;
    let groups = input.lines().array_chunks::<3>();
    'groups: for group in groups {
        let mut counts: HashMap<char, u32> = HashMap::new();
        for line in group {
            // We use a set for the line to not double-count any chars for the group
            let mut this_line: HashSet<char> = HashSet::new();

            for c in line.chars() {
                if !this_line.contains(&c) {
                    this_line.insert(c);
                    let v = counts.entry(c).or_insert(0);
                    if *v == 2 {
                        let priority = day_3_priority(c);
                        sum += priority;
                        continue 'groups;
                    } else {
                        *v += 1;
                    }
                }
            }
        }
    }
    Ok(sum.to_string())
}

fn day_4() -> Result<String> {
    let mut count = 0;
    let input = helper::load_puzzle_to_string(4, 1)?;
    for line in input.lines() {
        let mut parts = line.split(',');
        let mut e1 = parts.next().unwrap().split('-');
        let mut e2 = parts.next().unwrap().split('-');
        let e1_start: usize = e1.next().unwrap().parse().unwrap();
        let e1_end: usize = e1.next().unwrap().parse().unwrap();
        let e2_start: usize = e2.next().unwrap().parse().unwrap();
        let e2_end: usize = e2.next().unwrap().parse().unwrap();
        let e1_overlap = e1_start >= e2_start && e1_start <= e2_end;
        let e2_overlap = e2_start >= e1_start && e2_start <= e1_end;
        if e1_overlap || e2_overlap {
            count += 1;
        }
    }
    Ok(count.to_string())
}

struct Day5 {
    stacks: Vec<Vec<char>>,
}

impl Day5 {
    fn new() -> Self {
        Day5 { stacks: vec![] }
    }
    fn execute(&mut self, input: &str) -> Result<String> {
        for _ in 0..9 {
            self.stacks.push(vec![]);
        }
        let mut phase1 = true;
        for line in input.lines() {
            if line.is_empty() {
                phase1 = false;
                for stack in &mut self.stacks {
                    stack.reverse();
                }
                continue;
            }
            if phase1 {
                // Phase 1 - setup stacks
                let mut chars = line.chars().enumerate().peekable();
                while chars.peek().is_some() {
                    let (pos, c) = chars.next().unwrap();
                    println!("on {c}");
                    if c == '[' {
                        let (_, value) = chars.next().unwrap();
                        let stack_num = pos / 4;
                        println!("Pushing '{value}' to stack {stack_num}");
                        self.stacks[stack_num].push(value);
                        chars.next(); // Eat the ]
                    }
                }
            } else {
                println!("Phase 2: {line}");
                let words: Vec<&str> = line.split_whitespace().collect();
                let count: u32 = words[1].parse()?;
                let source_stack: usize = words[3].parse()?;
                let dest_stack: usize = words[5].parse()?;
                // self.do_move_part_1(count, source_stack - 1, dest_stack - 1);
                self.do_move_part_2(count, source_stack - 1, dest_stack - 1);
                println!("{self}");
            }
        }
        let results: String = self
            .stacks
            .iter()
            .map(|stack| stack.last().copied().unwrap_or('a'))
            .collect();
        Ok(results)
    }

    fn do_move_part_1(&mut self, count: u32, source_stack: usize, dest_stack: usize) {
        for _ in 0..count {
            let elem = self.stacks[source_stack].pop().unwrap();
            self.stacks[dest_stack].push(elem);
        }
    }

    fn do_move_part_2(&mut self, count: u32, source_stack: usize, dest_stack: usize) {
        let drained: Vec<char> = {
            let source_stack = &mut self.stacks[source_stack];
            let len = source_stack.len();
            source_stack.drain(len - count as usize..).collect()
        };
        for elem in drained {
            self.stacks[dest_stack].push(elem);
        }
    }
}

impl Display for Day5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, stack) in self.stacks.iter().enumerate() {
            f.write_fmt(format_args!("Stack {}: ", i + 1))?;
            for c in stack.iter() {
                f.write_fmt(format_args!("{c}, "))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn day_5() -> Result<String> {
    let input = helper::load_puzzle_to_string(5, 1)?;
    let mut state = Day5::new();
    let result = state.execute(&input)?;
    println!("{result}");
    Ok(result)
}

fn day6() -> Result<String> {
    let input = helper::load_puzzle_to_string(6, 1)?;
    let chars = input.chars().enumerate();
    let mut buf: VecDeque<char> = VecDeque::new();
    for (i, c) in chars {
        if buf.len() == 14 {
            buf.pop_front();
        }
        buf.push_back(c);
        // println!("At {c}, buf: {buf:?}");
        // let uniq: HashSet<char> = buf.iter().copied().collect().len();
        // let uniq: usize = buf.iter().unique().count();
        let uniq: bool = buf.iter().all_unique();
        if buf.len() == 14 && uniq {
            return Ok((i + 1).to_string());
        }
    }
    Ok("no signal ksssshzzzzzzzzkt".to_string())
}

