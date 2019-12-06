use std::io::stdin;

fn solve1(codes: Vec<i32>) -> i32 {
    let input = 1;
    let mut v = codes;
    let mut i = 0;
    loop {
        match v[i] % 100 {
            1 => {
                let lhs = if (v[i] / 100) % 10 == 0 {
                    v[v[i + 1] as usize]
                } else {
                    v[i + 1]
                };
                let rhs = if (v[i] / 1000) % 10 == 0 {
                    v[v[i + 2] as usize]
                } else {
                    v[i + 2]
                };
                let pos = v[i + 3] as usize;
                v[pos] = lhs + rhs;
                i += 4;
            }
            2 => {
                let lhs = if (v[i] / 100) % 10 == 0 {
                    v[v[i + 1] as usize]
                } else {
                    v[i + 1]
                };
                let rhs = if (v[i] / 1000) % 10 == 0 {
                    v[v[i + 2] as usize]
                } else {
                    v[i + 2]
                };
                let pos = v[i + 3] as usize;
                v[pos] = lhs * rhs;
                i += 4;
            }
            3 => {
                let pos = v[i + 1] as usize;
                v[pos] = input;
                i += 2;
            }
            4 => {
                v[0] = if v[i] > 100 {
                    v[i + 1]
                } else {
                    v[v[i + 1] as usize]
                };
                i += 2;
            }
            99 => break,
            _ => {}
        }
    }
    return v[0];
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let v: Vec<i32> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();
    let answer = solve1(v);
    println!("{}", answer);
}
