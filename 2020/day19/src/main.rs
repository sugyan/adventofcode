use std::collections::HashMap;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
enum Rule {
    Char(char),
    And(Vec<u8>),
    Or(Vec<u8>, Vec<u8>),
}

struct Solution {
    rules: HashMap<u8, Rule>,
    messages: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let mut rules: HashMap<u8, Rule> = HashMap::new();
        let mut messages: Vec<String> = Vec::new();
        for input in inputs.iter().filter(|&s| !s.is_empty()) {
            if input.starts_with(|c: char| c.is_numeric()) {
                let s: Vec<&str> = input.split(": ").collect();
                if let Ok(key) = s[0].parse() {
                    rules.insert(
                        key,
                        if s[1].starts_with('"') {
                            Rule::Char(s[1].chars().nth(1).unwrap())
                        } else if s[1].contains(" | ") {
                            let v: Vec<&str> = s[1].split(" | ").collect();
                            Rule::Or(
                                v[0].split(' ').filter_map(|s| s.parse().ok()).collect(),
                                v[1].split(' ').filter_map(|s| s.parse().ok()).collect(),
                            )
                        } else {
                            Rule::And(s[1].split(' ').filter_map(|s| s.parse().ok()).collect())
                        },
                    );
                }
            } else {
                messages.push(input.to_string());
            }
        }
        Self { rules, messages }
    }
    fn solve_1(&self) -> usize {
        self.messages
            .iter()
            .filter(|&message| {
                Solution::matches(message, &self.rules, 0)
                    .iter()
                    .any(|&s| s.is_empty())
            })
            .count()
    }
    fn solve_2(&self) -> usize {
        let mut rules = self.rules.clone();
        rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
        rules.insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));
        self.messages
            .iter()
            .filter(|&message| {
                Solution::matches(message, &rules, 0)
                    .iter()
                    .any(|&s| s.is_empty())
            })
            .count()
    }
    fn matches_all<'a>(
        message: &'a str,
        rules_map: &HashMap<u8, Rule>,
        rules: &[u8],
    ) -> Vec<&'a str> {
        let mut ret = vec![message];
        for &rule in rules.iter() {
            let mut next = Vec::new();
            for &message in ret.iter() {
                next.extend(Solution::matches(message, rules_map, rule));
            }
            ret = next;
        }
        ret
    }
    fn matches<'a>(message: &'a str, rules_map: &HashMap<u8, Rule>, rule: u8) -> Vec<&'a str> {
        if let Some(rule) = rules_map.get(&rule) {
            match rule {
                Rule::Char(c) => {
                    if Some(*c) == message.chars().next() {
                        vec![&message[1..]]
                    } else {
                        Vec::new()
                    }
                }
                Rule::And(v) => Solution::matches_all(message, rules_map, v),
                Rule::Or(r1, r2) => {
                    let mut ret = Vec::new();
                    ret.extend(Solution::matches_all(message, rules_map, r1));
                    ret.extend(Solution::matches_all(message, rules_map, r2));
                    ret
                }
            }
        } else {
            Vec::new()
        }
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::new(
                r#"
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }

    #[test]
    fn example_2() {
        let solution = Solution::new(
            r#"
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#[1..]
                .split('\n')
                .map(|s| s.to_string())
                .collect(),
        );
        assert_eq!(3, solution.solve_1());
        assert_eq!(12, solution.solve_2());
    }
}
