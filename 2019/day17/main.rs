use std::io::stdin;
use utils::IntCode;

struct Solution {
    input: String,
}

impl Solution {
    pub fn new(input: String) -> Solution {
        return Solution { input };
    }
    fn solve1(&self) -> i32 {
        let mut computer = IntCode::from(self.input.clone());
        let outputs = computer.run(vec![]);
        let ascii: String = outputs
            .iter()
            .map(|o| std::char::from_u32(*o as u32).unwrap())
            .collect();
        let mut v: Vec<Vec<char>> = Vec::new();
        for row in ascii.split('\n') {
            if !row.is_empty() {
                v.push(row.chars().collect());
            }
        }
        let mut answer = 0;
        for i in 1..v.len() - 1 {
            for j in 1..v[0].len() - 1 {
                if v[i][j] == '#'
                    && v[i - 1][j] == '#'
                    && v[i][j - 1] == '#'
                    && v[i + 1][j] == '#'
                    && v[i][j + 1] == '#'
                {
                    answer += i * j;
                }
            }
        }
        return answer as i32;
    }
    fn solve2(&self) {
        let mut codes: Vec<i64> = self
            .input
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect();
        codes[0] = 2;
        let mut computer = IntCode::new(codes);
        let outputs = computer.run(vec![]);
        let ascii: String = outputs
            .iter()
            .map(|o| std::char::from_u32(*o as u32).unwrap())
            .collect();
        println!("{}", ascii);
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();

    let solution = Solution::new(buf);
    // println!("{:?}", solution.solve1());
    solution.solve2();
}
