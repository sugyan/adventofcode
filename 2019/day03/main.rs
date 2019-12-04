use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io::stdin;

fn positions(s: &str) -> Vec<[i32; 2]> {
    let mut ret = vec![];
    let mut pos = [0, 0];
    for path in s.split(",") {
        let d = &path[..1];
        let n = &path[1..].parse().unwrap();
        let (mut dx, mut dy) = (0, 0);
        match d {
            "R" => dx = 1,
            "U" => dy = 1,
            "L" => dx = -1,
            "D" => dy = -1,
            &_ => {}
        }
        for _ in 0..*n {
            pos[0] += dx;
            pos[1] += dy;
            ret.push(pos);
        }
    }
    return ret;
}

fn solve1() -> i32 {
    let mut v: [Vec<[i32; 2]>; 2] = [vec![], vec![]];
    for i in 0..2 {
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        v[i] = positions(buf.trim());
    }
    let mut hs: HashSet<[i32; 2]> = HashSet::new();
    for pos in &v[0] {
        hs.insert(*pos);
    }
    let mut answer = std::i32::MAX;
    for pos in &v[1] {
        if let Some(pos) = hs.get(pos) {
            answer = std::cmp::min(answer, pos[0].abs() + pos[1].abs());
        }
    }
    return answer;
}

fn solve2() -> i32 {
    let mut v: [Vec<[i32; 2]>; 2] = [vec![], vec![]];
    for i in 0..2 {
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        v[i] = positions(buf.trim());
    }
    let mut hm: HashMap<[i32; 2], usize> = HashMap::new();
    for (i, pos) in (1..).zip(&v[0]) {
        if hm.get(pos).is_none() {
            hm.insert(*pos, i);
        }
    }
    let mut answer = std::i32::MAX;
    for (j, pos) in (1..).zip(&v[1]) {
        if let Some(i) = hm.get(pos) {
            answer = std::cmp::min(answer, (*i + j) as i32);
        }
    }
    return answer;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let answer = if args.len() < 2 || &args[1] != "2" {
        solve1()
    } else {
        solve2()
    };
    println!("{}", answer);
}
