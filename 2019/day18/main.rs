use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;

struct Solution {
    inputs: Vec<Vec<char>>,
}

impl Solution {
    pub fn new(input: Vec<String>) -> Solution {
        return Solution {
            inputs: input.iter().map(|s| s.chars().collect()).collect(),
        };
    }
    fn solve1(&self) -> usize {
        let dm: HashMap<char, Vec<(char, usize, u32)>> = self.calc_distances();
        let mut keys: Vec<&char> = dm.keys().filter(|k| k.is_lowercase()).collect();
        keys.sort();
        let mut hm: HashMap<char, HashMap<u32, usize>> = HashMap::new();
        let mut m: HashMap<u32, usize> = HashMap::new();
        m.insert(0, 0);
        hm.insert('@', m);
        for i in 0.. {
            let mut v: Vec<(char, u32, usize)> = Vec::new();
            for (src, m) in hm.iter() {
                for (ksrc, dist) in m.iter() {
                    let (mut count, mut k) = (0, *ksrc);
                    while k > 0 {
                        if k & 1 != 0 {
                            count += 1;
                        }
                        k >>= 1;
                    }
                    if count != i {
                        continue;
                    }
                    for (dst, d, kdst) in dm.get(src).unwrap().iter() {
                        if !dst.is_lowercase() {
                            continue;
                        }
                        if kdst & ksrc != *kdst {
                            continue;
                        }
                        let mut key = *ksrc;
                        let idx = keys.iter().position(|&k| *k == dst.to_ascii_lowercase());
                        if let Some(idx) = idx {
                            if ksrc & (1 << idx) != 0 {
                                continue;
                            }
                            key |= 1 << idx;
                        }
                        v.push((*dst, key, dist + *d));
                    }
                }
            }
            for (dst, k, d) in v.iter() {
                if let Some(m) = hm.get_mut(&dst) {
                    if let Some(e) = m.get_mut(k) {
                        *e = std::cmp::min(*e, *d);
                    } else {
                        m.insert(*k, *d);
                    }
                } else {
                    let mut m: HashMap<u32, usize> = HashMap::new();
                    m.insert(*k, *d);
                    hm.insert(*dst, m);
                }
            }

            let mut all: Vec<usize> = Vec::new();
            for v in hm.values() {
                for (key, val) in v.iter() {
                    if key + 1 == 1 << keys.len() {
                        all.push(*val);
                    }
                }
            }
            if let Some(answer) = all.iter().min() {
                return *answer;
            }
        }
        return 0;
    }
    fn calc_distances(&self) -> HashMap<char, Vec<(char, usize, u32)>> {
        let mut points: HashMap<(usize, usize), char> = HashMap::new();
        for i in 0..self.inputs.len() {
            for j in 0..self.inputs[0].len() {
                match self.inputs[i][j] {
                    '#' | '.' => {}
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
                if self.inputs[p.0 - 1][p.1] != '#' {
                    v.push((p.0 - 1, p.1));
                }
                if self.inputs[p.0][p.1 - 1] != '#' {
                    v.push((p.0, p.1 - 1));
                }
                if self.inputs[p.0 + 1][p.1] != '#' {
                    v.push((p.0 + 1, p.1));
                }
                if self.inputs[p.0][p.1 + 1] != '#' {
                    v.push((p.0, p.1 + 1));
                }
                for p in v {
                    if !hs.contains(&p) {
                        let mut k = k;
                        let c = self.inputs[p.0][p.1];
                        if c.is_alphabetic() {
                            if let Some(dst) = ret.get_mut(src) {
                                dst.push((c, d + 1, k));
                            } else {
                                ret.insert(*src, vec![(c, d + 1, k)]);
                            }
                            if c.is_uppercase() {
                                let idx = keys.iter().position(|&k| *k == c.to_ascii_lowercase());
                                if let Some(idx) = idx {
                                    k |= 1 << idx;
                                }
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
