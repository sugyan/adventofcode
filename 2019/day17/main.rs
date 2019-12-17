use std::io::stdin;
use utils::IntCode;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let mut computer = IntCode::from(buf);
    let outputs = computer.run(vec![]);
    let ascii: String = outputs
        .iter()
        .map(|o| std::char::from_u32(*o as u32).unwrap())
        .collect();
    // println!("{}", ascii);
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
    println!("{:?}", answer);
}
