use std::io::{BufRead, BufReader};

struct Solution {
    enhancement: Vec<bool>,
    image: Vec<Vec<bool>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let mut sections = inputs.split(String::is_empty);
        let enhancement = sections.next().unwrap()[0]
            .chars()
            .map(|c| c == '#')
            .collect();
        let image = sections
            .next()
            .unwrap()
            .iter()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { enhancement, image }
    }
    fn part_1(&self) -> usize {
        let len = self.image.len();
        let offset = 2;
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
        for k in 0..2 {
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
                                .fold(0, |acc, x| (acc << 1) + if x { 1 } else { 0 })]
                        })
                        .collect()
                })
                .collect();
        }
        image
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
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
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(35, Solution::new(&example_inputs()).part_1());
    }
}
