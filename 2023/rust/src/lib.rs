#[cfg(feature = "perf")]
use std::time::Instant;

pub trait Solve {
    type Answer1: std::fmt::Display;
    type Answer2: std::fmt::Display;

    fn new(r: impl std::io::Read) -> Self;
    fn part1(&self) -> Self::Answer1;
    fn part2(&self) -> Self::Answer2;
}

#[cfg(not(feature = "perf"))]
pub fn run(solution: &impl Solve) {
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(feature = "perf")]
pub fn run(solution: &impl Solve) {
    let (answer1, elapsed1) = {
        let now = Instant::now();
        let answer = solution.part1();
        let elapsed = now.elapsed();
        (answer, elapsed)
    };
    println!("Part 1: {answer1}");
    let (answer2, elapsed2) = {
        let now = Instant::now();
        let answer = solution.part2();
        let elapsed = now.elapsed();
        (answer, elapsed)
    };
    println!("Part 2: {answer2}");

    println!("--- Elapsed ---");
    println!("Part 1: {elapsed1:.3?}");
    println!("Part 2: {elapsed2:.3?}");
}
