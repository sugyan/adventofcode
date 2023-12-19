use aoc2023::Solve;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
struct Condition {
    category: Category,
    ord: Ordering,
    value: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SendTo {
    Accept,
    Reject,
    Other(String),
}

impl FromStr for SendTo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::Other(s.into())),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    condition: Option<Condition>,
    send_to: SendTo,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((condition, send_to)) = s.split_once(':') {
            let (category, ord) = match &condition[..2] {
                "x<" => (Category::X, Ordering::Less),
                "x>" => (Category::X, Ordering::Greater),
                "m<" => (Category::M, Ordering::Less),
                "m>" => (Category::M, Ordering::Greater),
                "a<" => (Category::A, Ordering::Less),
                "a>" => (Category::A, Ordering::Greater),
                "s<" => (Category::S, Ordering::Less),
                "s>" => (Category::S, Ordering::Greater),
                _ => return Err(()),
            };
            let value = condition[2..].parse().map_err(|_| ())?;
            Ok(Self {
                condition: Some(Condition {
                    category,
                    ord,
                    value,
                }),
                send_to: send_to.parse().map_err(|_| ())?,
            })
        } else {
            Ok(Self {
                condition: None,
                send_to: s.parse().map_err(|_| ())?,
            })
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules) = s.trim_end_matches('}').split_once('{').ok_or(())?;
        Ok(Self {
            name: name.into(),
            rules: rules
                .split(',')
                .map(|rule| rule.parse().map_err(|_| ()))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, m, a, s) = s
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .collect_tuple()
            .ok_or(())?;
        Ok(Self {
            x: x.strip_prefix("x=").ok_or(())?.parse().map_err(|_| ())?,
            m: m.strip_prefix("m=").ok_or(())?.parse().map_err(|_| ())?,
            a: a.strip_prefix("a=").ok_or(())?.parse().map_err(|_| ())?,
            s: s.strip_prefix("s=").ok_or(())?.parse().map_err(|_| ())?,
        })
    }
}

struct Solution {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let lines = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .collect::<Vec<_>>();
        let (workflows, parts) = lines.split(String::is_empty).collect_tuple().expect(
            "
        should be two parts",
        );
        Self {
            workflows: workflows
                .iter()
                .map(|line| line.parse().expect("should be valid workflow"))
                .collect(),
            parts: parts
                .iter()
                .map(|line| line.parse().expect("should be valid part"))
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let map = self
            .workflows
            .iter()
            .map(|w| (w.name.clone(), w.rules.clone()))
            .collect::<HashMap<_, _>>();
        let mut ret = 0;
        for part in &self.parts {
            let mut current = SendTo::Other("in".into());
            while let SendTo::Other(next) = &current {
                for rule in &map[next] {
                    if let Some(condition) = &rule.condition {
                        let v = match condition.category {
                            Category::X => part.x,
                            Category::M => part.m,
                            Category::A => part.a,
                            Category::S => part.s,
                        };
                        if v.cmp(&condition.value) == condition.ord {
                            current = rule.send_to.clone();
                            break;
                        }
                    } else {
                        current = rule.send_to.clone();
                    }
                }
            }
            if current == SendTo::Accept {
                ret += part.x + part.m + part.a + part.s;
            }
        }
        ret
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
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 19114);
    }
}
