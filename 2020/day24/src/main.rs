use std::collections::HashSet;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> usize {
        let mut flipped = HashSet::new();
        for input in self.inputs.iter() {
            let s: Vec<char> = input.chars().collect();
            let mut p = (0, 0);
            let mut i = 0;
            while i < s.len() {
                match s[i] {
                    'e' => p.0 += 2,
                    's' => {
                        p.1 -= 1;
                        match s[i + 1] {
                            'e' => p.0 += 1,
                            'w' => p.0 -= 1,
                            _ => unreachable!(),
                        }
                        i += 1
                    }
                    'w' => p.0 -= 2,
                    'n' => {
                        p.1 += 1;
                        match s[i + 1] {
                            'e' => p.0 += 1,
                            'w' => p.0 -= 1,
                            _ => unreachable!(),
                        }
                        i += 1
                    }
                    _ => unreachable!(),
                }
                i += 1;
            }
            if flipped.contains(&p) {
                flipped.remove(&p);
            } else {
                flipped.insert(p);
            }
        }
        flipped.len()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            10,
            Solution::new(
                "
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"[1..]
                    .split('\n')
                    .map(str::to_string)
                    .collect()
            )
            .solve_1()
        );
    }
}
