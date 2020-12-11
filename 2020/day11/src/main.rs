use std::io::{BufRead, BufReader};

struct Solution {
    layout: Vec<Vec<char>>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            layout: inputs.iter().map(|s| s.chars().collect()).collect(),
        }
    }
    fn solve_1(&self) -> usize {
        let positions: Vec<Vec<Vec<(usize, usize)>>> = self.positions(true);
        self.simulate(&positions, 4)
    }
    fn solve_2(&self) -> usize {
        let positions: Vec<Vec<Vec<(usize, usize)>>> = self.positions(false);
        self.simulate(&positions, 5)
    }
    fn positions(&self, adjacent: bool) -> Vec<Vec<Vec<(usize, usize)>>> {
        let (r, c) = (self.layout.len(), self.layout[0].len());
        let mut positions: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![Vec::with_capacity(8); c]; r];
        for (i, row) in positions.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if self.layout[i][j] == '.' {
                    continue;
                }
                for &d in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                {
                    for k in 1.. {
                        if adjacent && k > 1 {
                            break;
                        }
                        let ii = i as i32 + k * d.0;
                        let jj = j as i32 + k * d.1;
                        if ii < 0 || ii == r as i32 || jj < 0 || jj == c as i32 {
                            break;
                        }
                        if self.layout[ii as usize][jj as usize] != '.' {
                            col.push((ii as usize, jj as usize));
                            break;
                        }
                    }
                }
            }
        }
        positions
    }
    fn simulate(&self, positions: &[Vec<Vec<(usize, usize)>>], threshold: usize) -> usize {
        let mut curr = self.layout.clone();
        loop {
            let mut changed = false;
            let mut ret = 0;
            curr = curr
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(j, &col)| match col {
                            'L' => {
                                if positions[i][j]
                                    .iter()
                                    .filter(|&p| curr[p.0][p.1] == '#')
                                    .count()
                                    == 0
                                {
                                    changed = true;
                                    ret += 1;
                                    '#'
                                } else {
                                    'L'
                                }
                            }
                            '#' => {
                                if positions[i][j]
                                    .iter()
                                    .filter(|&p| curr[p.0][p.1] == '#')
                                    .count()
                                    >= threshold
                                {
                                    changed = true;
                                    'L'
                                } else {
                                    ret += 1;
                                    '#'
                                }
                            }
                            c => c,
                        })
                        .collect()
                })
                .collect();
            if !changed {
                return ret;
            }
        }
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
            37,
            Solution::new(
                "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            26,
            Solution::new(
                "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_2()
        );
    }
}
