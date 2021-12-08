use std::io::{BufRead, BufReader};

struct Solution {
    entries: Vec<(Vec<String>, Vec<String>)>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            entries: inputs
                .iter()
                .map(|s| {
                    let (patterns, output) = s.split_once(" | ").unwrap();
                    (
                        patterns.splitn(10, ' ').map(String::from).collect(),
                        output.splitn(4, ' ').map(String::from).collect(),
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
                    .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
                    .count()
            })
            .sum()
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
}
