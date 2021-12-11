use std::collections::VecDeque;
use std::io::{BufRead, BufReader};

struct Solution {
    energy_levels: Vec<Vec<u8>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            energy_levels: inputs
                .iter()
                .map(|row| row.bytes().map(|u| u - b'0').collect())
                .collect(),
        }
    }
    fn part_1(&self) -> u32 {
        let mut flashes = 0;
        let mut grid = self.energy_levels.clone();
        for _ in 0..100 {
            let mut vd = VecDeque::new();
            (0..10).for_each(|i| (0..10).for_each(|j| vd.push_back((i, j))));
            while let Some((i, j)) = vd.pop_front() {
                grid[i][j] += 1;
                if grid[i][j] == 10 {
                    [!0, 0, 1].iter().for_each(|&di| {
                        [!0, 0, 1].iter().for_each(|&dj| {
                            let i = i.wrapping_add(di);
                            let j = j.wrapping_add(dj);
                            if (0..10).contains(&i) && (0..10).contains(&j) {
                                vd.push_back((i, j));
                            }
                        });
                    });
                }
            }
            grid.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|col| {
                    if *col > 9 {
                        flashes += 1;
                        *col = 0;
                    }
                });
            });
        }
        flashes
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
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(1656, Solution::new(&example_inputs()).part_1());
    }
}
