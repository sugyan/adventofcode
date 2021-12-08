use std::io::{BufRead, BufReader};

struct Solution {
    entries: Vec<(Vec<u8>, Vec<u8>)>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let s2u8 = |s: &str| s.bytes().fold(0_u8, |acc, u| acc | 1 << (u - b'a'));
        Self {
            entries: inputs
                .iter()
                .map(|s| {
                    let (patterns, output) = s.split_once(" | ").unwrap();
                    (
                        patterns.splitn(10, ' ').map(s2u8).collect(),
                        output.splitn(4, ' ').map(s2u8).collect(),
                    )
                })
                .collect(),
        }
    }
    fn part_1(&self) -> usize {
        self.entries
            .iter()
            .map(|(_, o)| {
                o.iter()
                    .filter(|u| matches!(u.count_ones(), 2 | 3 | 4 | 7))
                    .count()
            })
            .sum()
    }
    fn part_2(&self) -> u32 {
        let get_output = |entry: &(Vec<u8>, Vec<u8>)| {
            let mut map = [0; 10];
            for &u in &entry.0 {
                match u.count_ones() {
                    2 => map[1] = u,
                    3 => map[7] = u,
                    4 => map[4] = u,
                    7 => map[8] = u,
                    _ => {}
                }
            }
            for &u in &entry.0 {
                match (
                    u.count_ones(),
                    (u & map[4]).count_ones(),
                    (u & map[7]).count_ones(),
                ) {
                    (5, 2, _) => map[2] = u,
                    (5, 3, 3) => map[3] = u,
                    (5, 3, 2) => map[5] = u,
                    (6, 4, _) => map[9] = u,
                    (6, 3, 3) => map[0] = u,
                    (6, 3, 2) => map[6] = u,
                    _ => {}
                }
            }
            entry
                .1
                .iter()
                .map(|&u| (0..=9).find(|&i| map[i as usize] == u).unwrap())
                .fold(0, |acc, u| acc * 10 + u)
        };
        self.entries.iter().map(get_output).sum()
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
    println!("{}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(26, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(61229, Solution::new(&example_inputs()).part_2());
    }
}
