use std::io::stdin;

struct Solution {
    inputs: Vec<i32>,
}

impl Solution {
    pub fn new(inputs: Vec<i32>) -> Solution {
        return Solution { inputs };
    }
    fn solve1(&self) -> i32 {
        let mut answer = 0;
        for n in self.inputs.iter() {
            answer += n / 3 - 2;
        }
        return answer;
    }
    fn solve2(&self) -> i32 {
        let mut answer = 0;
        for n in self.inputs.iter() {
            let mut m = *n;
            while m > 0 {
                m = m / 3 - 2;
                answer += std::cmp::max(0, m);
            }
        }
        return answer;
    }
}

fn main() {
    let mut inputs: Vec<i32> = Vec::new();
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        if buf.is_empty() {
            break;
        }
        if let Ok(n) = buf.trim().parse::<i32>() {
            inputs.push(n);
        }
    }
    let solution = Solution::new(inputs);
    println!("{}", solution.solve1());
    println!("{}", solution.solve2());
}
