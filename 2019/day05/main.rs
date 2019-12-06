use std::env;
use std::io::stdin;

fn solve(codes: Vec<i32>, input: i32) -> i32 {
    let mut v = codes;
    let mut i = 0;
    loop {
        let get_param = |pos: usize| -> i32 {
            return if (v[i] / 10i32.pow(pos as u32 + 1)) % 10 == 0 {
                v[v[i + pos] as usize]
            } else {
                v[i + pos]
            };
        };
        match v[i] % 100 {
            1 => {
                let pos = v[i + 3] as usize;
                v[pos] = get_param(1) + get_param(2);
                i += 4;
            }
            2 => {
                let pos = v[i + 3] as usize;
                v[pos] = get_param(1) * get_param(2);
                i += 4;
            }
            3 => {
                let pos = v[i + 1] as usize;
                v[pos] = input;
                i += 2;
            }
            4 => {
                v[0] = get_param(1);
                i += 2;
            }
            5 => {
                if get_param(1) != 0 {
                    i = get_param(2) as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                if get_param(1) == 0 {
                    i = get_param(2) as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                let pos = v[i + 3] as usize;
                v[pos] = if get_param(1) < get_param(2) { 1 } else { 0 };
                i += 4;
            }
            8 => {
                let pos = v[i + 3] as usize;
                v[pos] = if get_param(1) == get_param(2) { 1 } else { 0 };
                i += 4;
            }
            99 => break,
            _ => {}
        }
    }
    return v[0];
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let v: Vec<i32> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();
    let answer = if args.len() < 2 || &args[1] != "2" {
        solve(v, 1)
    } else {
        solve(v, 5)
    };
    println!("{}", answer);
}
