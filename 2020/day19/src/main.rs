use std::collections::HashMap;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
enum Rule {
    Ref(u8),
    Char(char),
    Sequence(Vec<Rule>),
    Or(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn matches<'a>(&self, message: &'a str, rules: &HashMap<u8, Rule>) -> Vec<&'a str> {
        match self {
            Rule::Ref(u) => {
                if let Some(rule) = rules.get(u) {
                    rule.matches(message, rules)
                } else {
                    Vec::new()
                }
            }
            Rule::Char(c) => {
                if Some(*c) == message.chars().next() {
                    vec![&message[1..]]
                } else {
                    Vec::new()
                }
            }
            Rule::Sequence(v) => {
                let mut ret = vec![message];
                for rule in v.iter() {
                    let mut messages = Vec::new();
                    for &m in ret.iter() {
                        messages.append(&mut rule.matches(m, rules));
                    }
                    ret = messages;
                }
                ret
            }
            Rule::Or(r1, r2) => {
                let mut ret = Vec::new();
                ret.append(&mut r1.as_ref().matches(message, rules));
                ret.append(&mut r2.as_ref().matches(message, rules));
                ret
            }
        }
    }
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
            if input.starts_with(char::is_numeric) {
                let s: Vec<&str> = input.split(": ").collect();
                if let Ok(key) = s[0].parse() {
                    rules.insert(
                        key,
                        if s[1].starts_with('"') {
                            Rule::Char(s[1].chars().nth(1).unwrap())
                        } else if s[1].contains(" | ") {
                            let v: Vec<&str> = s[1].split(" | ").collect();
                            Rule::Or(
                                Box::new(Rule::Sequence(
                                    v[0].split(' ')
                                        .filter_map(|s| s.parse().ok())
                                        .map(|n| Rule::Ref(n))
                                        .collect(),
                                )),
                                Box::new(Rule::Sequence(
                                    v[1].split(' ')
                                        .filter_map(|s| s.parse().ok())
                                        .map(|n| Rule::Ref(n))
                                        .collect(),
                                )),
                            )
                        } else {
                            Rule::Sequence(
                                s[1].split(' ')
                                    .filter_map(|s| s.parse().ok())
                                    .map(|n| Rule::Ref(n))
                                    .collect(),
                            )
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
        self.count_matches(&self.rules)
    }
    fn solve_2(&self) -> usize {
        let mut rules = self.rules.clone();
        rules.insert(
            8,
            Rule::Or(
                Box::new(Rule::Ref(42)),
                Box::new(Rule::Sequence(
                    [42, 8].iter().map(|&u| Rule::Ref(u)).collect(),
                )),
            ),
        );
        rules.insert(
            11,
            Rule::Or(
                Box::new(Rule::Sequence(
                    [42, 31].iter().map(|&u| Rule::Ref(u)).collect(),
                )),
                Box::new(Rule::Sequence(
                    [42, 11, 31].iter().map(|&u| Rule::Ref(u)).collect(),
                )),
            ),
        );
        self.count_matches(&rules)
    }
    fn count_matches(&self, rules: &HashMap<u8, Rule>) -> usize {
        if let Some(rule) = rules.get(&0) {
            self.messages
                .iter()
                .filter(|&message| rule.matches(message, &rules).into_iter().any(str::is_empty))
                .count()
        } else {
            0
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
