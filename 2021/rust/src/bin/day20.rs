use aoc2021::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    enhancement: Vec<bool>,
    image: Vec<Vec<bool>>,
}

impl Solution {
    fn enhance(&self, times: usize) -> Vec<Vec<bool>> {
        let len = self.image.len();
        let offset = times;
        let mut image = vec![vec![false; len + 2 * offset]; len + 2 * offset];
        for (i, row) in self.image.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                image[i + offset][j + offset] = col;
            }
        }
        let d = [
            (!0, !0),
            (!0, 0),
            (!0, 1),
            (0, !0),
            (0, 0),
            (0, 1),
            (1, !0),
            (1, 0),
            (1, 1),
        ];
        for k in 0..times {
            image = (0..len + 2 * offset)
                .map(|i| {
                    (0..len + 2 * offset)
                        .map(|j| {
                            self.enhancement[d
                                .iter()
                                .map(|&(di, dj)| {
                                    let i = i.wrapping_add(di);
                                    let j = j.wrapping_add(dj);
                                    if (0..len + 2 * offset).contains(&i)
                                        && (0..len + 2 * offset).contains(&j)
                                    {
                                        image[i][j]
                                    } else {
                                        self.enhancement[0] && k & 1 > 0
                                    }
                                })
                                .fold(0, |acc, x| (acc << 1) + usize::from(x))]
                        })
                        .collect()
                })
                .collect();
        }
        image
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        let inputs = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        let sections: (&[String], &[String]) = inputs.split(String::is_empty).next_tuple().unwrap();
        let enhancement = sections
            .0
            .first()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect();
        let image = sections
            .1
            .iter()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { enhancement, image }
    }
    fn part1(&self) -> Self::Answer1 {
        self.enhance(2)
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.enhance(50)
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
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
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"[1..].as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(35, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(3351, Solution::new(example_input()).part2());
    }
}
