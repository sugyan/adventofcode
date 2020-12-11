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
        let mut curr = self.layout.clone();
        let (r, c) = (self.layout.len(), self.layout[0].len());
        loop {
            let mut next = curr.clone();
            for i in 0..r {
                for j in 0..c {
                    let mut occupied = 0;
                    for ii in i as i32 - 1..=i as i32 + 1 {
                        for jj in j as i32 - 1..=j as i32 + 1 {
                            if (ii == i as i32 && jj == j as i32)
                                || ii < 0
                                || ii == r as i32
                                || jj < 0
                                || jj == c as i32
                            {
                                continue;
                            }
                            if curr[ii as usize][jj as usize] == '#' {
                                occupied += 1;
                            }
                        }
                    }
                    next[i][j] = match curr[i][j] {
                        'L' => {
                            if occupied == 0 {
                                '#'
                            } else {
                                'L'
                            }
                        }
                        '#' => {
                            if occupied >= 4 {
                                'L'
                            } else {
                                '#'
                            }
                        }
                        c => c,
                    }
                }
            }
            if curr == next {
                break;
            }
            curr = next;
        }
        curr.iter()
            .map(|v| v.iter().filter(|&c| *c == '#').count())
            .sum()
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
}
