use std::io::Read;

pub trait Solve {
    type Answer1;
    type Answer2;

    fn new(r: impl Read) -> Self;
    fn part1(&self) -> Self::Answer1;
    fn part2(&self) -> Self::Answer2;
}
