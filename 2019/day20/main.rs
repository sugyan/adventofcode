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
        let mut portals: HashMap<(usize, usize), String> = HashMap::new();
        for i in 0..self.inputs.len() {
            for j in 0..self.inputs[i].len() {
                if self.inputs[i][j].is_alphabetic() {
                    if i > 0 && self.inputs[i - 1][j].is_alphabetic() {
                        let portal = [self.inputs[i - 1][j], self.inputs[i][j]]
                            .iter()
                            .collect::<String>();
                        if i > 1 && self.inputs[i - 2][j] == '.' {
                            portals.insert((i - 2, j), portal);
                        } else {
                            portals.insert((i + 1, j), portal);
                        }
                    }
                    if j > 0 && self.inputs[i][j - 1].is_alphabetic() {
                        let portal = [self.inputs[i][j - 1], self.inputs[i][j]]
                            .iter()
                            .collect::<String>();
                        if j > 1 && self.inputs[i][j - 2] == '.' {
                            portals.insert((i, j - 2), portal);
                        } else {
                            portals.insert((i, j + 1), portal);
                        }
                    };
                }
            }
        }
        let mut dmap: HashMap<String, Vec<(String, usize)>> = HashMap::new();
        for (pos, src) in portals.iter() {
            let mut hs: HashSet<(usize, usize)> = HashSet::new();
            hs.insert(*pos);
            let mut q: VecDeque<((usize, usize), usize)> = VecDeque::new();
            q.push_back((*pos, 0));
            while !q.is_empty() {
                let mut v: Vec<(usize, usize)> = Vec::new();
                if let Some(front) = q.front() {
                    let (p, d) = *front;
                    if d > 0 {
                        if let Some(dst) = portals.get(&p) {
                            if let Some(v) = dmap.get_mut(src) {
                                v.push((dst.clone(), d));
                            } else {
                                dmap.insert(src.clone(), vec![(dst.clone(), d)]);
                            }
                        }
                    }
                    if p.0 > 0 && self.inputs[p.0 - 1][p.1] == '.' {
                        v.push((p.0 - 1, p.1));
                    }
                    if p.1 > 0 && self.inputs[p.0][p.1 - 1] == '.' {
                        v.push((p.0, p.1 - 1));
                    }
                    if p.0 < self.inputs.len() - 1 && self.inputs[p.0 + 1][p.1] == '.' {
                        v.push((p.0 + 1, p.1));
                    }
                    if p.1 < self.inputs[0].len() - 1 && self.inputs[p.0][p.1 + 1] == '.' {
                        v.push((p.0, p.1 + 1));
                    }
                    for pos in v {
                        if !hs.contains(&pos) {
                            hs.insert(pos);
                            q.push_back((pos, d + 1));
                        }
                    }
                }
                q.pop_front();
            }
        }
        let mut hm: HashMap<String, usize> = HashMap::new();
        hm.insert("AA".to_string(), 0);
        loop {
            let mut candidates: HashMap<String, usize> = HashMap::new();
            for (k, d) in hm.iter() {
                if let Some(v) = dmap.get(k) {
                    for e in v {
                        let d = d + e.1 + if *d == 0 { 0 } else { 1 };
                        if hm.contains_key(&e.0) {
                            continue;
                        }
                        if let Some(min) = candidates.get_mut(&e.0) {
                            *min = std::cmp::min(*min, d);
                        } else {
                            candidates.insert(e.0.to_string(), d);
                        }
                    }
                }
            }
            if candidates.is_empty() {
                break;
            }
            let (mut minkey, mut minval) = (String::new(), std::usize::MAX);
            for (k, v) in candidates.iter() {
                if *v < minval {
                    minkey = k.to_string();
                    minval = *v;
                }
            }
            hm.insert(minkey, minval);
        }
        return *hm.get("ZZ").unwrap();
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
        lines.push(buf.trim_end_matches('\n').to_string());
    }
    println!("{}", Solution::new(lines).solve1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input: Vec<String> = "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
"
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();
        assert_eq!(23, Solution::new(input).solve1());
    }

    #[test]
    fn example_2() {
        let input: Vec<String> = "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
"
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();
        assert_eq!(58, Solution::new(input).solve1());
    }
}
