use std::collections::{HashMap, HashSet};
use std::io::stdin;

fn solve1(v: &Vec<[usize; 2]>) -> (usize, usize) {
    let (mut answer, mut maxidx) = (0, 0);
    for i in 0..v.len() {
        let mut hs: HashSet<[i32; 2]> = HashSet::new();
        for j in 0..v.len() {
            if j == i {
                continue;
            }
            let d = [
                v[i][0] as i32 - v[j][0] as i32,
                v[i][1] as i32 - v[j][1] as i32,
            ];
            let mut ok = true;
            for e in hs.iter() {
                if d[0] * e[1] == e[0] * d[1]
                    && (d[0] < 0) == (e[0] < 0)
                    && (d[1] < 0) == (e[1] < 0)
                {
                    ok = false;
                    break;
                }
            }
            if ok {
                hs.insert(d);
            }
        }
        if hs.len() > answer {
            answer = hs.len();
            maxidx = i;
        }
    }
    return (answer, maxidx);
}

fn solve2(v: &Vec<[usize; 2]>, maxidx: usize) -> i32 {
    let mut hm: HashMap<[i32; 2], Vec<[i32; 2]>> = HashMap::new();
    for j in 0..v.len() {
        if j == maxidx {
            continue;
        }
        let d = [
            v[j][0] as i32 - v[maxidx][0] as i32,
            v[j][1] as i32 - v[maxidx][1] as i32,
        ];
        let mut ok = true;
        for (key, value) in hm.iter_mut() {
            if d[0] * key[1] == key[0] * d[1]
                && (d[0] < 0) == (key[0] < 0)
                && (d[1] < 0) == (key[1] < 0)
            {
                value.push(d);
                ok = false;
                break;
            }
        }
        if ok {
            hm.insert(d, vec![d]);
        }
    }
    let mut count = 0;
    for values in hm.values_mut() {
        count += values.len();
        values.sort_by_cached_key(|v| v[0].abs() + v[1].abs());
    }
    let mut keys: Vec<[i32; 2]> = hm.keys().map(|v| v.clone()).collect();
    keys.sort_by(|a, b| {
        let f1 = (a[0] as f64).atan2(a[1] as f64);
        let f2 = (b[0] as f64).atan2(b[1] as f64);
        return f2.partial_cmp(&f1).unwrap();
    });
    let (mut answer, mut i) = (0, 1);
    while i <= count {
        for key in keys.iter() {
            if let Some(values) = hm.get_mut(key) {
                if let Some(first) = values.first() {
                    if i == 200 {
                        let x = v[maxidx][0] as i32 + first[0];
                        let y = v[maxidx][1] as i32 + first[1];
                        answer = x * 100 + y;
                    }
                    values.remove(0);
                    i += 1;
                }
            }
        }
    }
    return answer;
}

fn main() {
    let mut v: Vec<[usize; 2]> = Vec::new();
    for y in 0.. {
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        if buf.len() == 0 {
            break;
        }
        for (x, c) in (0..).zip(buf.chars()) {
            if c == '#' {
                v.push([x, y]);
            }
        }
    }
    let (answer1, maxidx) = solve1(&v);
    println!("{} ({})", answer1, maxidx);
    let answer2 = solve2(&v, maxidx);
    println!("{}", answer2);
}
