use std::io::{BufRead, BufReader};

struct Solution {
    scores: Vec<(usize, u32)>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let numbers = inputs[0]
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<u32>>();
        let mut map = vec![0; *numbers.iter().max().unwrap() as usize + 1];
        numbers.iter().enumerate().for_each(|(i, &n)| {
            map[n as usize] = i;
        });
        let boards = inputs[2..]
            .split(String::is_empty)
            .map(|v| {
                v.iter()
                    .map(|s| {
                        s.split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let scores = boards
            .iter()
            .map(|b| {
                let mapped = b
                    .iter()
                    .map(|row| row.iter().map(|&col| map[col as usize]).collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                let min = std::cmp::min(
                    (0..5)
                        .map(|i| (0..5).map(|j| mapped[i][j]).max().unwrap())
                        .min()
                        .unwrap(),
                    (0..5)
                        .map(|i| (0..5).map(|j| mapped[j][i]).max().unwrap())
                        .min()
                        .unwrap(),
                );
                let score = b
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|&col| if map[col as usize] > min { col } else { 0 })
                            .sum::<u32>()
                    })
                    .sum::<u32>();
                (min, score * numbers[min])
            })
            .collect::<Vec<_>>();
        Self { scores }
    }
    fn part_1(&self) -> u32 {
        self.scores.iter().min_by_key(|(i, _)| i).unwrap().1
    }
    fn part_2(&self) -> u32 {
        self.scores.iter().max_by_key(|(i, _)| i).unwrap().1
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
    println!("{}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(4512, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(1924, Solution::new(&example_inputs()).part_2());
    }
}
