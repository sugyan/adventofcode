use aoc2023::Solve;
use std::collections::HashMap;
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
                .map(str::parse)
                .collect::<Result<_, _>>()
                .map_err(|_| ())?,
        })
    }
}

struct Solution {
    rows: Vec<Row>,
}

impl Solution {
    fn possible_arrangements(s: &[char], target: &[usize]) -> u64 {
        Self::possible_arrangements_with_memo(s, target, &mut HashMap::new())
    }
    fn possible_arrangements_with_memo(
        s: &[char],
        target: &[usize],
        memo: &mut HashMap<(Vec<char>, Vec<usize>), u64>,
    ) -> u64 {
        if target.is_empty() {
            return u64::from(!s.contains(&'#'));
        }
        let key = (s.to_vec(), target.to_vec());
        if let Some(ret) = memo.get(&key) {
            return *ret;
        }
        let mut ret = 0;
        for i in 0..s.len() {
            let s = &s[i..];
            if s.len() < target[0] {
                break;
            }
            if s.len() == target[0] {
                ret += u64::from(target.len() == 1 && !s.contains(&'.'));
            } else if !s[0..target[0]].contains(&'.') && s[target[0]] != '#' {
                ret +=
                    Self::possible_arrangements_with_memo(&s[target[0] + 1..], &target[1..], memo);
            }
            if s[0] == '#' {
                break;
            }
        }
        memo.insert(key, ret);
        ret
    }
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;

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
                Self::possible_arrangements(&row.springs.chars().collect::<Vec<_>>(), &row.groups)
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.rows
            .iter()
            .map(|row| {
                Self::possible_arrangements(
                    &[row.springs.as_str(); 5]
                        .join("?")
                        .chars()
                        .collect::<Vec<_>>(),
                    &row.groups.repeat(5),
                )
            })
            .sum()
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

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 525_152);
    }
}
