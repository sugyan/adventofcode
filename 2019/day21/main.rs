use std::io::stdin;
use utils::IntCode;

struct Solution {
    codes: Vec<i64>,
}

impl Solution {
    pub fn new(input: String) -> Solution {
        return Solution {
            codes: input
                .split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect(),
        };
    }
    pub fn solve1(&self) -> i32 {
        let mut computer = IntCode::new(self.codes.clone());

        let script = "
NOT A T
OR T J
NOT B T
OR T J
NOT C T
OR T J
NOT D T
NOT T T
AND T J
        ";
        let mut lines: Vec<String> = script
            .split('\n')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        lines.push("WALK".to_string());
        let inputs: Vec<i64> = (lines.join("\n") + "\n")
            .chars()
            .map(|c| c as i64)
            .collect();
        let outputs = computer.run(inputs);
        return *outputs.last().unwrap() as i32;
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();

    let solution = Solution::new(buf);
    println!("{}", solution.solve1());
}
