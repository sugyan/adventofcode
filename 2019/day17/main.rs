use std::collections::HashSet;
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
        let mut computer = IntCode::new(self.codes.clone());
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
    fn solve2(&self) -> i32 {
        let mut codes = self.codes.clone();
        codes[0] = 2;
        let mut computer = IntCode::new(codes);
        let outputs = computer.run(vec![]);
        let ascii: String = outputs
            .iter()
            .map(|o| std::char::from_u32(*o as u32).unwrap())
            .collect();
        let lines: Vec<String> = ascii.split('\n').map(|s| s.to_string()).collect();
        let v: Vec<Vec<char>> = lines
            .iter()
            .filter(|s| s.len() == lines[0].len())
            .map(|s| s.chars().collect())
            .collect();
        let (mut x, mut y) = (0, 0);
        for i in 0..v.len() {
            for j in 0..v[0].len() {
                if v[i][j] != '.' && v[i][j] != '#' {
                    x = j;
                    y = i;
                }
            }
        }
        let d = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut didx = match v[y][x] {
            '^' => 0,
            '>' => 1,
            'v' => 2,
            '<' => 3,
            _ => std::usize::MAX,
        };
        let mut path = String::new();
        let mut count = 0;
        loop {
            let i = y as i32 + d[didx].1;
            let j = x as i32 + d[didx].0;
            if (i >= 0 && j >= 0)
                && (i < v.len() as i32 && j < v[0].len() as i32)
                && v[i as usize][j as usize] == '#'
            {
                count += 1;
                x = (x as i32 + d[didx].0) as usize;
                y = (y as i32 + d[didx].1) as usize;
                continue;
            }
            {
                let i = y as i32 + d[(didx + 1) % 4].1;
                let j = x as i32 + d[(didx + 1) % 4].0;
                if (i >= 0 && j >= 0)
                    && (i < v.len() as i32 && j < v[0].len() as i32)
                    && v[i as usize][j as usize] == '#'
                {
                    if count > 0 {
                        path.push_str(&count.to_string());
                        path.push(',')
                    }
                    path.push('R');
                    count = 0;
                    didx = (didx + 1) % 4;
                    continue;
                }
            }
            {
                let i = y as i32 + d[(didx + 3) % 4].1;
                let j = x as i32 + d[(didx + 3) % 4].0;
                if (i >= 0 && j >= 0)
                    && (i < v.len() as i32 && j < v[0].len() as i32)
                    && v[i as usize][j as usize] == '#'
                {
                    if count > 0 {
                        path.push_str(&count.to_string());
                        path.push(',')
                    }
                    path.push('L');
                    count = 0;
                    didx = (didx + 3) % 4;
                    continue;
                }
            }
            path.push_str(&count.to_string());
            break;
        }
        let mut delimiters: Vec<usize> = Vec::new();
        for (i, c) in (0..).zip(path.chars()) {
            if c == ',' {
                delimiters.push(i);
            }
        }
        let mut abc = vec![String::new(); 3];
        'search: for i in 0..delimiters.len() - 1 {
            for j in i + 1..delimiters.len() {
                if delimiters[i] > 20 || delimiters[j] - delimiters[i] > 20 {
                    continue;
                }
                let a = &path[0..delimiters[i]];
                let b = &path[delimiters[i] + 1..delimiters[j]];
                let p = if a.len() > b.len() {
                    path.clone().replace(a, "").replace(b, "")
                } else {
                    path.clone().replace(b, "").replace(a, "")
                };
                let c = p
                    .split(",,")
                    .map(|s| s.trim_start_matches(',').trim_end_matches(','))
                    .filter(|s| !s.is_empty())
                    .collect::<HashSet<&str>>();
                if c.len() == 1 {
                    abc[0] = a.to_string();
                    abc[1] = b.to_string();
                    abc[2] = c.iter().nth(0).unwrap().to_string();
                    break 'search;
                }
            }
        }
        abc.sort_by_cached_key(|f| f.len());
        for (i, f) in (0..).zip(abc.iter().rev()) {
            path = path.replace(f, vec!["A", "B", "C"][2 - i]);
        }
        for f in abc.iter_mut() {
            *f = f.replace("R", "R,").replace("L", "L,");
        }
        let mut input: String = path + "\n";
        input.push_str((abc.join("\n") + "\n").as_str());
        input.push_str("n\n");

        let outputs = computer.run(input.chars().map(|c| c as i64).collect());
        return *outputs.last().unwrap() as i32;
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();

    let solution = Solution::new(buf);
    println!("{}", solution.solve1());
    println!("{}", solution.solve2());
}
