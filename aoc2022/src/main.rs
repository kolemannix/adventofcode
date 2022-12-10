#![feature(iter_array_chunks)]

use anyhow::{Error, Result};
use std::collections::HashMap;
mod day7;
mod helper;
mod past;

fn day8() -> Result<String> {
    let input = helper::load_puzzle_to_string(8, 1)?;
    let mut grid: Vec<Vec<u32>> = vec![];
    for (line_num, line) in input.lines().enumerate() {
        grid.push(vec![]);
        for c in line.chars() {
            let num: u32 = c.to_digit(10).unwrap();
            grid[line_num].push(num);
        }
    }
    let mut sum_seen = 0;
    let mut scenic_scores: HashMap<(usize, usize), u32> = HashMap::new();
    // TODO: A dynamic programming solution that leverages the optimal substucture, namely:
    // If my neighbor is visible, and I am greater than my neighbor, I am visible.
    // If my neighbor is not visible, and I am less than or equal to my neighbor, then I am not
    // visible.
    'outer: for (y, line) in grid.iter().enumerate() {
        // These lines to skip edges are not needed for part 2, but do not affect the solution
        // since the best tree is not on an edge
        if y == 0 || y == grid.len() - 1 {
            sum_seen += line.len();
            continue 'outer;
        }
        'inner: for (x, n) in line.iter().enumerate() {
            // These lines to skip edges are not needed for part 2, but do not affect the solution
            // since the best tree is not on an edge
            if x == 0 || x == line.len() - 1 {
                sum_seen += 1;
                continue 'inner;
            }
            // Can we see a way out from here for c
            // Up
            let mut up_score = 0;
            let mut my_y: usize = y;
            loop {
                if my_y == 0 {
                    break;
                }
                my_y -= 1;
                up_score += 1;
                let next = grid[my_y][x];
                println!("UP    for {x},{y}: {n}: {next}");
                if next >= *n {
                    break;
                }
            }
            // Down
            let mut down_score = 0;
            my_y = y;
            loop {
                if my_y == grid.len() - 1 {
                    break;
                }
                my_y += 1;
                down_score += 1;
                let next = grid[my_y][x];
                println!("DOWN  for {x},{y}: {n}: {next}");
                if next >= *n {
                    break;
                }
            }
            // Left
            let mut left_score = 0;
            let mut my_x = x;
            loop {
                if my_x == 0 {
                    break;
                }
                my_x -= 1;
                left_score += 1;
                let next = grid[y][my_x];
                println!("LEFT  for {x},{y}: {n}: {next}");
                if next >= *n {
                    break;
                }
            }
            // Right
            let mut right_score = 0;
            let mut my_x = x;
            loop {
                if my_x == line.len() - 1 {
                    break;
                }
                my_x += 1;
                right_score += 1;
                let next = grid[y][my_x];
                println!("RIGHT for {x},{y}: {n}: {next}");
                if next >= *n {
                    break;
                }
            }
            scenic_scores.insert((x, y), up_score * down_score * left_score * right_score);
            // if up_vis || down_vis || left_vis || right_vis {
            //     println!("VISIBLE {x},{y}: {n}");
            //     sum_seen += 1;
            // }
        }
    }
    // println!("Part 1: {sum_seen}");
    // println!("{scenic_scores:?}");
    let max = scenic_scores.into_values().max().unwrap();
    Ok(max.to_string())
}

fn main() -> Result<()> {
    let res = day8()?;
    println!("{res}");
    Ok(())
}
