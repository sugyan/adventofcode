use std::io::{BufRead, BufReader};

struct Solution {
    lines: Vec<String>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            lines: inputs.to_vec(),
        }
    }
    fn part_1(&self) -> u32 {
        self.lines
            .iter()
            .map(|line| {
                let mut stack = Vec::new();
                for c in line.chars() {
                    match c {
                        ')' if stack.last() == Some(&'(') => {
                            stack.pop();
                        }
                        ')' => return 3,
                        ']' if stack.last() == Some(&'[') => {
                            stack.pop();
                        }
                        ']' => return 57,
                        '}' if stack.last() == Some(&'{') => {
                            stack.pop();
                        }
                        '}' => return 1197,
                        '>' if stack.last() == Some(&'<') => {
                            stack.pop();
                        }
                        '>' => return 25137,
                        c => stack.push(c),
                    }
                }
                0
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
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(26397, Solution::new(&example_inputs()).part_1());
    }
}
