use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    map: HashMap<String, Vec<String>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let mut map = HashMap::new();
        for input in inputs {
            let caves = input.split_once('-').unwrap();
            if caves.1 != "start" {
                map.entry(caves.0.to_string())
                    .or_insert_with(Vec::new)
                    .push(caves.1.to_string());
            }
            if caves.0 != "start" {
                map.entry(caves.1.to_string())
                    .or_insert_with(Vec::new)
                    .push(caves.0.to_string());
            }
        }
        Self { map }
    }
    fn part_1(&self) -> u32 {
        self.backtrack(
            String::from("start"),
            &mut vec![String::from("start")],
            false,
        )
    }
    fn part_2(&self) -> u32 {
        self.backtrack(
            String::from("start"),
            &mut vec![String::from("start")],
            true,
        )
    }
    fn backtrack(&self, src: String, path: &mut Vec<String>, twice: bool) -> u32 {
        if src == "end" {
            return 1;
        }
        let mut count = 0;
        if let Some(v) = self.map.get(&src) {
            for dst in v {
                let mut twice = twice;
                if *dst == dst.to_lowercase() && path.contains(dst) {
                    if twice {
                        twice = false;
                    } else {
                        continue;
                    }
                }
                path.push(dst.to_string());
                count += self.backtrack(dst.to_string(), path, twice);
                path.pop();
            }
        }
        count
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

    fn example_inputs() -> Vec<Vec<String>> {
        vec![
            r"
start-A
start-b
A-c
A-b
b-d
A-end
b-end"[1..]
                .split('\n')
                .map(String::from)
                .collect(),
            r"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"[1..]
                .split('\n')
                .map(String::from)
                .collect(),
            r"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"[1..]
                .split('\n')
                .map(String::from)
                .collect(),
        ]
    }

    #[test]
    fn example_1() {
        for (inputs, expected) in example_inputs().iter().zip(vec![10, 19, 226]) {
            assert_eq!(expected, Solution::new(inputs).part_1());
        }
    }

    #[test]
    fn example_2() {
        for (inputs, expected) in example_inputs().iter().zip(vec![36, 103, 3509]) {
            assert_eq!(expected, Solution::new(inputs).part_2());
        }
    }
}
