use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    layout: Vec<Vec<char>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            layout: inputs.iter().map(|s| s.chars().collect()).collect(),
        }
    }
    fn part_1(&self) -> usize {
        let target_seats = self.target_seats(true);
        self.simulate(&target_seats, 4)
    }
    fn part_2(&self) -> usize {
        let target_seats = self.target_seats(false);
        self.simulate(&target_seats, 5)
    }
    fn target_seats(&self, adjacent: bool) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
        let (r, c) = (self.layout.len() as i32, self.layout[0].len() as i32);
        let mut seats = HashMap::new();
        for (i, row) in self.layout.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                if col == '.' {
                    continue;
                }
                for &d in &[
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    for k in 1.. {
                        if adjacent && k > 1 {
                            break;
                        }
                        let ii = i as i32 + k * d.0;
                        let jj = j as i32 + k * d.1;
                        if ii < 0 || ii == r || jj < 0 || jj == c {
                            break;
                        }
                        if self.layout[ii as usize][jj as usize] != '.' {
                            seats
                                .entry((i, j))
                                .or_insert_with(Vec::new)
                                .push((ii as usize, jj as usize));
                            break;
                        }
                    }
                }
            }
        }
        seats
    }
    fn simulate(
        &self,
        target_seats: &HashMap<(usize, usize), Vec<(usize, usize)>>,
        threshold: usize,
    ) -> usize {
        let mut curr = self.layout.clone();
        loop {
            let next = curr
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(j, &col)| {
                            if col == '.' {
                                col
                            } else {
                                let count = target_seats.get(&(i, j)).map_or(0, |v| {
                                    v.iter().filter(|&p| curr[p.0][p.1] == '#').count()
                                });
                                match col {
                                    'L' if count == 0 => '#',
                                    '#' if count >= threshold => 'L',
                                    c => c,
                                }
                            }
                        })
                        .collect()
                })
                .collect();
            if next == curr {
                return curr
                    .iter()
                    .map(|row| row.iter().filter(|&c| *c == '#').count())
                    .sum();
            }
            curr = next
        }
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

    fn example_inputs() -> Vec<String> {
        r"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(37, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(26, Solution::new(&example_inputs()).part_2());
    }
}
