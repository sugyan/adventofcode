use std::{
    error::Error,
    fmt::Display,
    io::{BufRead, stdin},
};

pub trait Day {
    type Input;
    type Answer1: Display;
    type Answer2: Display;
    type Error: Error;

    fn parse<R: BufRead>(r: R) -> Result<Self::Input, Self::Error>;
    fn part1(input: &Self::Input) -> Self::Answer1;
    fn part2(input: &Self::Input) -> Self::Answer2;
}

pub trait Solve: Sized {
    type Answer1: Display;
    type Answer2: Display;
    type Error: Error;

    fn new<R>(r: R) -> Result<Self, Self::Error>
    where
        R: BufRead;
    fn part1(&self) -> Self::Answer1;
    fn part2(&self) -> Self::Answer2;
}

pub fn run_day_with<D, R>(r: R) -> Result<(), D::Error>
where
    D: Day,
    R: BufRead,
{
    let data = D::parse(r)?;
    println!("Part 1: {}", D::part1(&data));
    println!("Part 2: {}", D::part2(&data));
    Ok(())
}

pub fn run_day<D>() -> Result<(), D::Error>
where
    D: Day,
{
    run_day_with::<D, _>(stdin().lock())
}

pub fn run_with<S, R>(r: R) -> Result<(), S::Error>
where
    S: Solve,
    R: BufRead,
{
    let solution = S::new(r)?;
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
    Ok(())
}

pub fn run<S>() -> Result<(), S::Error>
where
    S: Solve,
{
    run_with::<S, _>(stdin().lock())
}
