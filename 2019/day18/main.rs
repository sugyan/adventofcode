use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;

struct Solution {
    inputs: RefCell<Vec<Vec<char>>>,
}

impl Solution {
    pub fn new(input: Vec<String>) -> Solution {
        return Solution {
            inputs: RefCell::new(input.iter().map(|s| s.chars().collect()).collect()),
        };
    }
    fn solve1(&self) -> usize {
        let dm: HashMap<char, Vec<(char, usize, u32)>> = Solution::calc_map(&self.inputs.borrow());
        let target: u32 = (1 << dm.keys().filter(|c| c.is_alphabetic()).count()) - 1;
        let mut hm: HashMap<u32, HashMap<char, usize>> = HashMap::new();
        let mut m: HashMap<char, usize> = HashMap::new();
        m.insert('@', 0);
        hm.insert(0, m);
        loop {
            let mut next: HashMap<u32, HashMap<char, usize>> = HashMap::new();
            for (k, values) in hm.iter() {
                for (src, min) in values.iter() {
                    for (dst, d, keys) in dm.get(src).unwrap().iter() {
                        if keys & k != *keys {
                            continue;
                        }
                        let key = k | (1 << (*dst as u8 - 'a' as u8) as usize);
                        if key == *k {
                            continue;
                        }
                        if let Some(m) = next.get_mut(&key) {
                            if let Some(e) = m.get_mut(dst) {
                                *e = std::cmp::min(*e, min + d);
                            } else {
                                m.insert(*dst, min + d);
                            }
                        } else {
                            let mut m: HashMap<char, usize> = HashMap::new();
                            m.insert(*dst, min + d);
                            next.insert(key, m);
                        }
                    }
                }
            }
            hm = next;
            if let Some(v) = hm.get(&target) {
                return *v.values().min().unwrap();
            }
        }
    }
    fn calc_map(inputs: &Vec<Vec<char>>) -> HashMap<char, Vec<(char, usize, u32)>> {
        let mut points: HashMap<(usize, usize), char> = HashMap::new();
        for i in 0..inputs.len() {
            for j in 0..inputs[0].len() {
                match inputs[i][j] {
                    '#' | '.' | 'A'..='Z' => {}
                    c => {
                        points.insert((i, j), c);
                    }
                }
            }
        }
        let mut keys: Vec<&char> = points.values().filter(|c| c.is_lowercase()).collect();
        keys.sort();
        let mut ret: HashMap<char, Vec<(char, usize, u32)>> = HashMap::new();
        for (pos, src) in points.iter() {
            let mut hs: HashSet<(usize, usize)> = HashSet::new();
            let mut q: VecDeque<((usize, usize), usize, u32)> = VecDeque::new();
            hs.insert(*pos);
            q.push_back((*pos, 0, 0));
            while let Some(front) = q.front() {
                let (p, d, k) = *front;
                let mut v: Vec<(usize, usize)> = Vec::new();
                if inputs[p.0 - 1][p.1] != '#' {
                    v.push((p.0 - 1, p.1));
                }
                if inputs[p.0][p.1 - 1] != '#' {
                    v.push((p.0, p.1 - 1));
                }
                if inputs[p.0 + 1][p.1] != '#' {
                    v.push((p.0 + 1, p.1));
                }
                if inputs[p.0][p.1 + 1] != '#' {
                    v.push((p.0, p.1 + 1));
                }
                for p in v {
                    if !hs.contains(&p) {
                        let mut k = k;
                        let c = inputs[p.0][p.1];
                        if c.is_alphabetic() {
                            if c.is_lowercase() {
                                if let Some(dst) = ret.get_mut(src) {
                                    dst.push((c, d + 1, k));
                                } else {
                                    ret.insert(*src, vec![(c, d + 1, k)]);
                                }
                            } else {
                                k |= 1 << (c as u8 - 'A' as u8) as usize;
                            }
                        }
                        q.push_back((p, d + 1, k));
                        hs.insert(p);
                    }
                }
                q.pop_front();
            }
        }
        return ret;
    }
}

fn main() {
    let mut lines: Vec<String> = Vec::new();
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        if buf.is_empty() {
            break;
        }
        lines.push(buf.trim().to_string());
    }
    let solution = Solution::new(lines);
    println!("{}", solution.solve1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let inputs: Vec<String> = "
#########
#b.A.@.a#
#########
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
        assert_eq!(8, Solution::new(inputs).solve1());
    }

    #[test]
    fn example_2() {
        let inputs: Vec<String> = "
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
        assert_eq!(86, Solution::new(inputs).solve1());
    }

    #[test]
    fn example_3() {
        let inputs: Vec<String> = "
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
        assert_eq!(132, Solution::new(inputs).solve1());
    }

    #[test]
    fn example_4() {
        let inputs: Vec<String> = "
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
        assert_eq!(136, Solution::new(inputs).solve1());
    }

    #[test]
    fn example_5() {
        let inputs: Vec<String> = "
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
"
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
        assert_eq!(81, Solution::new(inputs).solve1());
    }
}
