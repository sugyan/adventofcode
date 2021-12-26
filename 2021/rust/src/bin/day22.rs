use itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

type Step = (
    Turn,
    (
        RangeInclusive<i32>,
        RangeInclusive<i32>,
        RangeInclusive<i32>,
    ),
);

#[derive(Debug)]
enum Turn {
    On,
    Off,
}

struct Solution {
    steps: Vec<Step>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            steps: inputs
                .iter()
                .map(|input| {
                    let (turn, ranges) = input.split_once(' ').unwrap();
                    (
                        match turn {
                            "on" => Turn::On,
                            "off" => Turn::Off,
                            _ => unreachable!(),
                        },
                        ranges
                            .split(',')
                            .map(|s| {
                                let (start, end) = s[2..]
                                    .split("..")
                                    .map(|s| s.parse().unwrap())
                                    .collect_tuple()
                                    .unwrap();
                                start..=end
                            })
                            .collect_tuple()
                            .unwrap(),
                    )
                })
                .collect(),
        }
    }
    fn part_1(&self) -> usize {
        let mut grid = vec![vec![vec![false; 101]; 101]; 101];
        for (turn, (x, y, z)) in &self.steps {
            if *x.start() < -50
                || *x.end() > 50
                || *y.start() < -50
                || *y.end() > 50
                || *z.start() < -50
                || *z.end() > 50
            {
                continue;
            }
            for i in *x.start()..=*x.end() {
                let i = (i + 50) as usize;
                for j in *y.start()..=*y.end() {
                    let j = (j + 50) as usize;
                    for k in *z.start()..=*z.end() {
                        let k = (k + 50) as usize;
                        match turn {
                            Turn::On => grid[i][j][k] = true,
                            Turn::Off => grid[i][j][k] = false,
                        }
                    }
                }
            }
        }
        grid.iter().flatten().flatten().filter(|&&b| b).count()
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
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(590_784, Solution::new(&example_inputs()).part_1());
    }
}
