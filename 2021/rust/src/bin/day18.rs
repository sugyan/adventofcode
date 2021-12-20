use std::io::{BufRead, BufReader};

struct Solution {
    numbers: Vec<Vec<(u32, usize)>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            numbers: inputs
                .iter()
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
    fn part_1(&self) -> u32 {
        let mut number = self.numbers[0].clone();
        Self::reduce(&mut number);
        number = self.numbers.iter().skip(1).fold(number, |mut acc, x| {
            acc.extend(x.iter());
            acc.iter_mut().for_each(|x| x.1 += 1);
            Self::reduce(&mut acc);
            acc
        });
        Self::magnitude(&mut number);
        number[0].0
    }
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
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(4140, Solution::new(&example_inputs()).part_1());
    }
}
