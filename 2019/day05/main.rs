use std::io::stdin;

fn solve1(codes: Vec<i32>) -> i32 {
    let mut v = codes;
    return v[0];
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let v: Vec<i32> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();
    let answer = solve1(v);
}
