use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

struct Solution {
    paths: [Vec<(i32, i32)>; 2],
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let path = |s: &String| -> Vec<(i32, i32)> {
            s.split(',')
                .flat_map(|s| {
                    std::iter::repeat(match &s[..1] {
                        "R" => (1, 0),
                        "U" => (0, 1),
                        "L" => (-1, 0),
                        "D" => (0, -1),
                        _ => unreachable!(),
                    })
                    .take(s[1..].parse().unwrap_or(0))
                })
                .scan((0, 0), |pos, d| {
                    pos.0 += d.0;
                    pos.1 += d.1;
                    Some(*pos)
                })
                .collect()
        };
        Self {
            paths: [path(&inputs[0]), path(&inputs[1])],
        }
    }
    fn part_1(&self) -> i32 {
        let hs0 = self.paths[0].iter().collect::<HashSet<_>>();
        let hs1 = self.paths[1].iter().collect::<HashSet<_>>();
        hs0.intersection(&hs1)
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap()
    }
    fn part_2(&self) -> i32 {
        let hm = (1..)
            .zip(self.paths[0].iter())
            .fold(HashMap::new(), |mut hm, (i, pos)| {
                hm.entry(pos).or_insert(i);
                hm
            });
        (1..)
            .zip(self.paths[1].iter())
            .filter_map(|(i, pos)| hm.get(pos).map(|&j| i + j))
            .min()
            .unwrap()
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs_1() -> Vec<String> {
        r"
R8,U5,L5,D3
U7,R6,D4,L4"
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    fn example_inputs_2() -> Vec<String> {
        r"
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    fn example_inputs_3() -> Vec<String> {
        r"
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::new(&&example_inputs_1()).part_1());
        assert_eq!(159, Solution::new(&&example_inputs_2()).part_1());
        assert_eq!(135, Solution::new(&&example_inputs_3()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(30, Solution::new(&&example_inputs_1()).part_2());
        assert_eq!(610, Solution::new(&&example_inputs_2()).part_2());
        assert_eq!(410, Solution::new(&&example_inputs_3()).part_2());
    }
}
