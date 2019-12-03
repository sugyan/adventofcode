use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let mut hs: HashSet<[i32; 2]> = HashSet::new();
    {
        let mut pos = [0, 0];
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        for path in buf.trim().split(",") {
            let d: &str = &path[..1];
            let n: &i32 = &path[1..].parse().unwrap();
            match d {
                "R" => {
                    for _ in 0..*n {
                        pos[0] += 1;
                        hs.insert(pos);
                    }
                }
                "U" => {
                    for _ in 0..*n {
                        pos[1] += 1;
                        hs.insert(pos);
                    }
                }
                "L" => {
                    for _ in 0..*n {
                        pos[0] -= 1;
                        hs.insert(pos);
                    }
                }
                "D" => {
                    for _ in 0..*n {
                        pos[1] -= 1;
                        hs.insert(pos);
                    }
                }
                &_ => {}
            }
        }
    }

    let mut answer = std::i32::MAX;
    {
        let mut pos = [0, 0];
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        for path in buf.trim().split(",") {
            let d: &str = &path[..1];
            let n: &i32 = &path[1..].parse().unwrap();
            match d {
                "R" => {
                    for _ in 0..*n {
                        pos[0] += 1;
                        if let Some(pos) = hs.get(&pos) {
                            answer = std::cmp::min(answer, pos[0].abs() + pos[1].abs());
                        }
                    }
                }
                "U" => {
                    for _ in 0..*n {
                        pos[1] += 1;
                        if let Some(pos) = hs.get(&pos) {
                            answer = std::cmp::min(answer, pos[0].abs() + pos[1].abs());
                        }
                    }
                }
                "L" => {
                    for _ in 0..*n {
                        pos[0] -= 1;
                        if let Some(pos) = hs.get(&pos) {
                            answer = std::cmp::min(answer, pos[0].abs() + pos[1].abs());
                        }
                    }
                }
                "D" => {
                    for _ in 0..*n {
                        pos[1] -= 1;
                        if let Some(pos) = hs.get(&pos) {
                            answer = std::cmp::min(answer, pos[0].abs() + pos[1].abs());
                        }
                    }
                }
                &_ => {}
            }
        }
    }
    println!("{}", answer);
}
