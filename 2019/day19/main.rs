use std::io::stdin;
use utils::IntCode;

struct Solution {}

impl Solution {
    fn solve1(input: String) -> i32 {
        let codes: Vec<i64> = input
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let mut answer = 0;
        for i in 0..50 {
            for j in 0..50 {
                let output = IntCode::new(codes.clone()).run(vec![i, j]);
                if output[0] == 1 {
                    answer += 1;
                }
            }
        }
        return answer;
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    println!("{}", Solution::solve1(buf));
}
