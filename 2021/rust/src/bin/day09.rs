use std::io::{BufRead, BufReader};

struct Solution {
    heightmap: Vec<Vec<u8>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            heightmap: inputs
                .iter()
                .map(|line| line.bytes().map(|u| u - b'0').collect())
                .collect(),
        }
    }
    fn part_1(&self) -> u32 {
        let (r, c) = (self.heightmap.len(), self.heightmap[0].len());
        self.heightmap
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, &h)| {
                        if (i == 0 || self.heightmap[i - 1][j] > h)
                            && (i == r - 1 || self.heightmap[i + 1][j] > h)
                            && (j == 0 || self.heightmap[i][j - 1] > h)
                            && (j == c - 1 || self.heightmap[i][j + 1] > h)
                        {
                            h as u32 + 1
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
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
2199943210
3987894921
9856789892
8767896789
9899965678"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(15, Solution::new(&example_inputs()).part_1());
    }
}
