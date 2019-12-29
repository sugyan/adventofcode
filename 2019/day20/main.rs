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
        let dmap = self.calc_distances();
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
    fn solve2(&self) -> usize {
        let dmap = self.calc_distances();
        let mut hm: HashMap<(String, usize), usize> = HashMap::new();
        hm.insert(("AA".to_string(), 0), 0);
        loop {
            let mut candidates: HashMap<(String, usize), usize> = HashMap::new();
            for (k, d) in hm.iter() {
                if let Some(v) = dmap.get(&k.0) {
                    for e in v {
                        let d = d + e.1 + if *d == 0 { 0 } else { 1 };
                        let key = (e.0.to_string(), (k.1 as i32 + e.2) as usize);
                        if e.0 == "ZZ" && key.1 > 0 {
                            continue;
                        }
                        if e.0 != "ZZ" && key.1 == 0 {
                            continue;
                        }
                        if hm.contains_key(&key) {
                            continue;
                        }
                        if let Some(min) = candidates.get_mut(&key) {
                            *min = std::cmp::min(*min, d);
                        } else {
                            candidates.insert(key, d);
                        }
                    }
                }
            }
            let (mut minkey, mut minval) = ((String::new(), 0), std::usize::MAX);
            for (k, v) in candidates.iter() {
                if *v < minval {
                    minkey = k.clone();
                    minval = *v;
                }
            }
            hm.insert(minkey, minval);
            let mut found = false;
            for key in hm.keys() {
                if key.0 == "ZZ" {
                    found = true;
                }
            }
            if found {
                break;
            }
        }
        return *hm.get(&("ZZ".to_string(), 0)).unwrap();
    }
    fn calc_distances(&self) -> HashMap<String, Vec<(String, usize, i32)>> {
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
        let mut dmap: HashMap<String, Vec<(String, usize, i32)>> = HashMap::new();
        for (pos, src) in portals.iter() {
            let src_out = pos.0 == 2
                || pos.1 == 2
                || pos.0 == self.inputs.len() - 3
                || pos.1 == self.inputs[0].len() - 3;
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
                            let dst_out = p.0 == 2
                                || p.1 == 2
                                || p.0 == self.inputs.len() - 3
                                || p.1 == self.inputs[0].len() - 3;
                            let level = if src_out == dst_out {
                                0
                            } else if dst_out {
                                -1
                            } else {
                                1
                            };
                            if let Some(v) = dmap.get_mut(src) {
                                v.push((dst.clone(), d, level));
                            } else {
                                dmap.insert(src.clone(), vec![(dst.clone(), d, level)]);
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
        return dmap;
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
    let solution = Solution::new(lines);
    println!("{}", solution.solve1());
    println!("{}", solution.solve2());
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
        let solution = Solution::new(input);
        assert_eq!(23, solution.solve1());
        assert_eq!(26, solution.solve2());
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

    #[test]
    fn example_3() {
        let input: Vec<String> = "
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     
"
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();
        assert_eq!(396, Solution::new(input).solve2());
    }
}
