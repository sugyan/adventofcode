use std::collections::HashMap;
use std::env;
use std::io::stdin;

fn count(hs: &HashMap<String, Vec<String>>, src: String, depth: i32) -> i32 {
    let mut ret = depth;
    if let Some(dst) = hs.get(&src) {
        for d in dst {
            ret += count(hs, d.clone(), depth + 1);
        }
    }
    return ret;
}

fn solve1(orbits: Vec<[String; 2]>) -> i32 {
    let mut hs: HashMap<String, Vec<String>> = HashMap::new();
    for orbit in orbits {
        let (src, dst) = (orbit[0].clone(), orbit[1].clone());
        if let Some(d) = hs.get_mut(&src) {
            d.push(dst);
        } else {
            hs.insert(src, vec![dst]);
        }
    }
    return count(&hs, "COM".to_string(), 0);
}

fn solve2(orbits: Vec<[String; 2]>) -> i32 {
    let mut hs: HashMap<String, String> = HashMap::new();
    for orbit in orbits {
        let (src, dst) = (orbit[0].clone(), orbit[1].clone());
        hs.insert(dst, src);
    }
    let path = |start: String| -> Vec<String> {
        let mut ret = Vec::new();
        let mut src = &start;
        while let Some(dst) = hs.get(src) {
            ret.push(src.clone());
            src = &dst;
        }
        ret.reverse();
        return ret;
    };
    let mut vyou = path("YOU".to_string());
    let mut vsan = path("SAN".to_string());
    while vyou[0] == vsan[0] {
        vyou.remove(0);
        vsan.remove(0);
    }
    return (vyou.len() + vsan.len()) as i32 - 2;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut orbits: Vec<[String; 2]> = Vec::new();
    loop {
        let mut buf = String::new();
        if stdin().read_line(&mut buf).unwrap() == 0 {
            break;
        }
        let orbit: Vec<&str> = buf.trim().splitn(2, ")").collect();
        orbits.push([orbit[0].to_string(), orbit[1].to_string()]);
    }
    let answer = if args.len() < 2 || &args[1] != "2" {
        solve1(orbits)
    } else {
        solve2(orbits)
    };
    println!("{}", answer);
}
