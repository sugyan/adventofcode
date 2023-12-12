use aoc2023::Solve;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Row {
    springs: String,
    groups: Vec<usize>,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) = s.split_once(' ').ok_or(())?;
        Ok(Self {
            springs: springs.to_string(),
            groups: groups
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<_, _>>()
                .map_err(|_| ())?,
        })
    }
}

struct Solution {
    rows: Vec<Row>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            rows: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| line.parse().expect("should be valid row"))
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.rows
            .iter()
            .map(|row| {
                let mut count = 0;
                let mut vd = VecDeque::from([row.springs.clone()]);
                while let Some(s) = vd.pop_front() {
                    if let Some((l, r)) = s.split_once('?') {
                        vd.extend([format!("{l}.{r}"), format!("{l}#{r}")]);
                    } else if s
                        .split('.')
                        .filter_map(|s| if s.is_empty() { None } else { Some(s.len()) })
                        .collect::<Vec<_>>()
                        == row.groups
                    {
                        count += 1;
                    }
                }
                count
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 21);
    }
}
