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
    fn solve1(&self) -> i32 {
        let mut answer = 0;
        for i in 0..50 {
            for j in 0..50 {
                let output = IntCode::new(self.codes.clone()).run(vec![i, j]);
                if output[0] == 1 {
                    answer += 1;
                }
            }
        }
        return answer;
    }
    fn solve2(self) -> i32 {
        // let mut v: Vec<Vec<i64>> = vec![vec![0; 20]; 20];
        let (mut x, mut y) = (0, 0);
        'nearest: for i in 0..10 {
            for j in 0..10 {
                if i == 0 && j == 0 {
                    continue;
                }
                let output = IntCode::new(self.codes.clone()).run(vec![i, j]);
                if output[0] == 1 {
                    x = i;
                    y = j;
                    break 'nearest;
                }
            }
        }
        'search: loop {
            loop {
                let output = IntCode::new(self.codes.clone()).run(vec![x, y]);
                match output[0] {
                    0 => {
                        y += 1;
                        break;
                    }
                    1 => {
                        if x >= 99 {
                            let output = IntCode::new(self.codes.clone()).run(vec![x - 99, y + 99]);
                            if output[0] == 1 {
                                break 'search;
                            }
                        }
                        x += 1;
                    }
                    _ => {}
                }
            }
        }
        return (x as i32 - 99) * 10000 + y as i32;
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let solution = Solution::new(buf);
    println!("{}", solution.solve1());
    println!("{}", solution.solve2());
}
