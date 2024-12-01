use std::{
    error::Error,
    fmt::Display,
    io::{stdin, Read},
};

pub trait Solve: Sized {
    type Answer1: Display;
    type Answer2: Display;
    type Error: Error;

    fn new(r: impl Read) -> Result<Self, Self::Error>;
    fn part1(&self) -> Self::Answer1;
    fn part2(&self) -> Self::Answer2;
}

pub fn run<S>()
where
    S: Solve,
{
    let solution = S::new(stdin().lock()).expect("failed to read input");
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}
