use std::{
    fmt::Display,
    io::{BufRead, stdin},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error<E> {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(E),
}

pub trait Day {
    type Input;
    type Error: std::error::Error + 'static;
    type Answer1: Display;
    type Answer2: Display;

    fn part1(input: &Self::Input) -> Self::Answer1;
    fn part2(input: &Self::Input) -> Self::Answer2;
}

pub trait DayParsed: Day {
    fn parse<R>(r: R) -> Result<Self::Input, Error<Self::Error>>
    where
        R: BufRead;
}

impl<T> DayParsed for T
where
    T: Day,
    T::Input: FromStr<Err = T::Error>,
{
    fn parse<R: BufRead>(mut r: R) -> Result<Self::Input, Error<Self::Error>> {
        let mut s = String::new();
        r.read_to_string(&mut s).map_err(Error::Io)?;
        s.parse().map_err(Error::Parse)
    }
}

pub trait Solve: Sized {
    type Answer1: Display;
    type Answer2: Display;
    type Error: std::error::Error;

    fn new<R>(r: R) -> Result<Self, Self::Error>
    where
        R: BufRead;
    fn part1(&self) -> Self::Answer1;
    fn part2(&self) -> Self::Answer2;
}

pub fn run_day_with<D, R>(mut r: R) -> Result<(), Error<D::Error>>
where
    D: DayParsed,
    R: BufRead,
{
    let data = D::parse(&mut r)?;
    println!("Part 1: {}", D::part1(&data));
    println!("Part 2: {}", D::part2(&data));
    Ok(())
}

pub fn run_day<D>() -> Result<(), Error<D::Error>>
where
    D: DayParsed,
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
