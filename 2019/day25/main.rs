use std::fs::read_to_string;
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
    fn solve1(&self) {
        let mut droid = IntCode::new(self.codes.clone());
        let mut inputs: Vec<i64> = Vec::new();
        loop {
            let outputs = droid.run(inputs);
            let ascii: String = outputs
                .iter()
                .map(|a| std::char::from_u32(*a as u32).unwrap())
                .collect();
            println!("{}", ascii);

            let mut buf = String::new();
            stdin().read_line(&mut buf).ok();
            if buf.trim() == "quit" {
                break;
            }
            inputs = buf.chars().map(|c| c as i64).collect();
        }
    }
}

fn main() {
    if let Ok(s) = read_to_string("input.txt") {
        let solution = Solution::new(s);
        println!("{}", solution.solve1());
    }
}
