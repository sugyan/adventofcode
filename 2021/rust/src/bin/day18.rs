use aoc2021::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    numbers: Vec<Vec<(u32, usize)>>,
}

impl Solution {
    fn reduce(number: &mut Vec<(u32, usize)>) {
        if number.iter().all(|&(n, d)| n < 10 && d < 5) {
            return;
        }
        for i in 0..number.len() - 1 {
            if number[i].1 >= 5 && number[i + 1].1 == number[i].1 {
                if i > 0 {
                    number[i - 1].0 += number[i].0;
                }
                if i + 2 < number.len() {
                    number[i + 2].0 += number[i + 1].0;
                }
                number[i] = (0, number[i].1 - 1);
                number.remove(i + 1);
                return Self::reduce(number);
            }
        }
        for i in 0..number.len() {
            if number[i].0 > 9 {
                number.insert(i, (number[i].0 / 2, number[i].1 + 1));
                number[i + 1].0 -= number[i].0;
                number[i + 1].1 += 1;
                return Self::reduce(number);
            }
        }
    }
    fn magnitude(number: &mut Vec<(u32, usize)>) {
        if number.len() < 2 {
            return;
        }
        for i in 0..number.len() - 1 {
            if number[i].1 == number[i + 1].1 {
                number[i] = (number[i].0 * 3 + number[i + 1].0 * 2, number[i].1 - 1);
                number.remove(i + 1);
                return Self::magnitude(number);
            }
        }
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            numbers: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|s| {
                    s.bytes()
                        .fold((Vec::new(), 0), |mut acc, u| {
                            match u {
                                b'[' => acc.1 += 1,
                                b']' => acc.1 -= 1,
                                b',' => {}
                                u => acc.0.push(((u - b'0') as u32, acc.1)),
                            }
                            acc
                        })
                        .0
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut number = self.numbers[0].clone();
        Self::reduce(&mut number);
        number = self.numbers.iter().skip(1).fold(number, |mut acc, x| {
            acc.extend(x.iter());
            acc.iter_mut().for_each(|(_, d)| *d += 1);
            Self::reduce(&mut acc);
            acc
        });
        Self::magnitude(&mut number);
        number[0].0
    }
    fn part2(&self) -> Self::Answer2 {
        (0..self.numbers.len())
            .flat_map(move |i| {
                (0..self.numbers.len()).filter_map(move |j| {
                    if i != j {
                        let mut number = std::iter::empty()
                            .chain(self.numbers[i].iter().map(|&(n, d)| (n, d + 1)))
                            .chain(self.numbers[j].iter().map(|&(n, d)| (n, d + 1)))
                            .collect();
                        Self::reduce(&mut number);
                        Self::magnitude(&mut number);
                        Some(number[0].0)
                    } else {
                        None
                    }
                })
            })
            .max()
            .unwrap()
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
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(4140, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(3993, Solution::new(example_input()).part2());
    }
}
