use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut answer = 0;
    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            if let Ok(mass) = line.trim().parse::<u32>() {
                answer += mass / 3 - 2;
            }
        }
    }
    println!("{}", answer);
}
