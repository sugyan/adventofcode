use std::collections::HashSet;
use std::io::{BufRead, BufReader};

struct Solution {
    flipped: HashSet<(i32, i32)>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let mut flipped = HashSet::new();
        for input in inputs.iter() {
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
        Self { flipped }
    }
    fn solve_1(&self) -> usize {
        self.flipped.len()
    }
    fn solve_2(&self) -> usize {
        let mut flipped = self.flipped.clone();
        let adjacents = [(2, 0), (1, -1), (-1, -1), (-2, 0), (-1, 1), (1, 1)];
        for _ in 0..100 {
            let mut targets = HashSet::new();
            flipped.iter().for_each(|&p| {
                targets.insert(p);
                adjacents.iter().for_each(|&d| {
                    targets.insert((p.0 + d.0, p.1 + d.1));
                })
            });
            flipped = targets
                .into_iter()
                .filter(|&p| {
                    let count = adjacents
                        .iter()
                        .filter(|&d| flipped.contains(&(p.0 + d.0, p.1 + d.1)))
                        .count();
                    count == 2 || (count == 1 && flipped.contains(&p))
                })
                .collect();
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
    println!("{}", solution.solve_2());
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

    #[test]
    fn example_2() {
        assert_eq!(
            2208,
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
            .solve_2()
        );
    }
}
