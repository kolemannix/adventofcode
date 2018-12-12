use crate::file::load_file;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn parse_claim(s: &str) -> Result<Claim, std::num::ParseIntError> {
    let re = Regex::new(r"^#([0-9]*) @ ([0-9]*),([0-9]*): ([0-9]*)x([0-9]*)$").unwrap();
    let cap = re.captures(s).unwrap();
    Ok(Claim {
        id: cap[1].parse::<u32>()?,
        x: cap[2].parse::<u32>()?,
        y: cap[3].parse::<u32>()?,
        width: cap[4].parse::<u32>()?,
        height: cap[5].parse::<u32>()?,
    })
}

pub fn part1() -> i32 {
    let problem = load_file("src/day3/day3input.txt");
    let mut occupied: HashMap<(u32, u32), u32> = HashMap::new();

    for s in &problem {
        let claim = parse_claim(s).unwrap();
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                match occupied.get_mut(&(x, y)) {
                    Some(v) => *v = *v + 1,
                    None => {
                        occupied.insert((x, y), 1);
                    }
                }
            }
        }
        // println!("{:?}", occupied);
    }
    let mut best_claim = None;
    for s in &problem {
        let claim = parse_claim(s).unwrap();
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                let mut no_overclaimed = true;
                if occupied.contains_key(&(x, y)) {
                    no_overclaimed = false;
                }
                if !no_overclaimed {
                    let b = Box::new(claim);
                    best_claim = Some(b);
                }
            }
        }
    }
    let mut overclaimed = 0;
    for (_, v) in occupied {
        if v > 1 {
            overclaimed = overclaimed + 1;
        }
    }
    println!("{:?}", overclaimed);
    println!("{:?}", best_claim);
    overclaimed
}

pub fn part2() -> i32 {
    42
}
