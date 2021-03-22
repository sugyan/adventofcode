use std::env;
use std::io::stdin;

fn solve1(a: i32, b: i32) -> i32 {
    let mut answer = 0;
    for i in a..b + 1 {
        let v = [
            (i / 10i32.pow(5)) % 10,
            (i / 10i32.pow(4)) % 10,
            (i / 10i32.pow(3)) % 10,
            (i / 10i32.pow(2)) % 10,
            (i / 10i32.pow(1)) % 10,
            (i / 10i32.pow(0)) % 10,
        ];
        if v[0] > v[1] || v[1] > v[2] || v[2] > v[3] || v[3] > v[4] || v[4] > v[5] {
            continue;
        }
        if v[0] == v[1] || v[1] == v[2] || v[2] == v[3] || v[3] == v[4] || v[4] == v[5] {
            answer += 1;
        }
    }
    return answer;
}

fn solve2(a: i32, b: i32) -> i32 {
    let mut answer = 0;
    for i in a..b + 1 {
        let v = [
            (i / 10i32.pow(5)) % 10,
            (i / 10i32.pow(4)) % 10,
            (i / 10i32.pow(3)) % 10,
            (i / 10i32.pow(2)) % 10,
            (i / 10i32.pow(1)) % 10,
            (i / 10i32.pow(0)) % 10,
        ];
        if v[0] > v[1] || v[1] > v[2] || v[2] > v[3] || v[3] > v[4] || v[4] > v[5] {
            continue;
        }
        for i in 0..5 {
            if v[i] == v[i + 1] {
                if (i == 0 || v[i - 1] != v[i]) && (i >= 4 || v[i + 2] != v[i + 1]) {
                    answer += 1;
                    break;
                }
            }
        }
    }
    return answer;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let v: Vec<i32> = buf
        .splitn(2, "-")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let answer = if args.len() < 2 || &args[1] != "2" {
        solve1(v[0], v[1])
    } else {
        solve2(v[0], v[1])
    };
    println!("{}", answer);
}
