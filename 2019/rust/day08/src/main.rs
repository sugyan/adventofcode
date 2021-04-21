use std::io::{BufRead, BufReader};

const W: usize = 25;
const H: usize = 6;

struct Solution {
    layers: Vec<Vec<Vec<u8>>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            layers: inputs[0]
                .as_bytes()
                .chunks(W * H)
                .map(|chunk| chunk.chunks(H).map(|row| row.into()).collect())
                .collect(),
        }
    }
    fn part_1(&self) -> i32 {
        self.layers
            .iter()
            .map(|layer| {
                layer.iter().flatten().fold([0, 0, 0], |mut acc, &x| {
                    match x {
                        b'0' => acc[0] += 1,
                        b'1' => acc[1] += 1,
                        b'2' => acc[2] += 1,
                        _ => {}
                    }
                    acc
                })
            })
            .min_by_key(|counts| counts[0])
            .map(|counts| counts[1] * counts[2])
            .unwrap()
    }
    fn part_2(&self) -> i32 {
        unimplemented!()
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
