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
            map.entry(caves.0.to_string())
                .or_insert_with(Vec::new)
                .push(caves.1.to_string());
            map.entry(caves.1.to_string())
                .or_insert_with(Vec::new)
                .push(caves.0.to_string());
        }
        Self { map }
    }
    fn part_1(&self) -> u32 {
        self.backtrack(String::from("start"), &mut vec![String::from("start")])
    }
    fn backtrack(&self, src: String, path: &mut Vec<String>) -> u32 {
        if src == "end" {
            return 1;
        }
        let mut count = 0;
        if let Some(v) = self.map.get(&src) {
            for dst in v {
                if *dst == dst.to_lowercase() && path.contains(dst) {
                    continue;
                }
                path.push(dst.to_string());
                count += self.backtrack(dst.to_string(), path);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            10,
            Solution::new(
                &r"
start-A
start-b
A-c
A-b
b-d
A-end
b-end"[1..]
                    .split('\n')
                    .map(String::from)
                    .collect::<Vec<_>>()
            )
            .part_1()
        );
        assert_eq!(
            19,
            Solution::new(
                &r"
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
                    .collect::<Vec<_>>()
            )
            .part_1()
        );
        assert_eq!(
            226,
            Solution::new(
                &r"
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
                    .collect::<Vec<_>>()
            )
            .part_1()
        );
    }
}
