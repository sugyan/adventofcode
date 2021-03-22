use std::io::stdin;
use utils::IntCode;

struct Solution {
    codes: Vec<i64>,
}
impl Solution {
    pub fn new(input: String) -> Solution {
        let codes: Vec<i64> = input
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect();
        return Solution { codes };
    }
    fn solve1(&self) -> i64 {
        let mut codes = self.codes.clone();
        codes[1] = 12;
        codes[2] = 02;
        let mut computer = IntCode::new(codes);
        computer.run(vec![]);
        return computer.get_val(0);
    }
    fn solve2(&self) -> i64 {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut codes = self.codes.clone();
                codes[1] = noun;
                codes[2] = verb;
                let mut computer = IntCode::new(codes);
                computer.run(vec![]);
                if computer.get_val(0) == 19690720 {
                    return noun * 100 + verb;
                }
            }
        }
        return -1;
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let solution = Solution::new(buf);
    println!("{}", solution.solve1());
    println!("{}", solution.solve2());
}
