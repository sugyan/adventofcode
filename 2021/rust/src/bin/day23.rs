use std::io::{BufRead, BufReader};

struct Solution {}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {}
    }
    fn part_1(&self) -> u32 {
        unimplemented!()
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(12521, Solution::new(&example_inputs()).part_1());
    }
}
