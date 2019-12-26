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
    fn solve1(&self) -> i32 {
        let mut portals: HashMap<(usize, usize), String> = HashMap::new();
        let h = self.inputs.len() - 4;
        let w = self.inputs[0].len() - 4;
        let mut field: Vec<Vec<char>> = self
            .inputs
            .iter()
            .skip(2)
            .take(h)
            .map(|s| {
                s.iter()
                    .skip(2)
                    .take(w)
                    .map(|c| if c.is_alphabetic() { ' ' } else { *c })
                    .collect()
            })
            .collect();
        for i in 0..self.inputs.len() {
            for j in 0..self.inputs[i].len() {
                if self.inputs[i][j].is_alphabetic() {
                    if i > 0 && self.inputs[i - 1][j].is_alphabetic() {
                        let portal = [self.inputs[i - 1][j], self.inputs[i][j]]
                            .iter()
                            .collect::<String>();
                        if i > 1 && self.inputs[i - 2][j] == '.' {
                            portals.insert((i - 4, j - 2), portal);
                        } else {
                            portals.insert((i - 1, j - 2), portal);
                        }
                    }
                    if j > 0 && self.inputs[i][j - 1].is_alphabetic() {
                        let portal = [self.inputs[i][j - 1], self.inputs[i][j]]
                            .iter()
                            .collect::<String>();
                        if j > 1 && self.inputs[i][j - 2] == '.' {
                            portals.insert((i - 2, j - 4), portal);
                        } else {
                            portals.insert((i - 2, j - 1), portal);
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
                    if p.0 > 0 && field[p.0 - 1][p.1] == '.' {
                        v.push((p.0 - 1, p.1));
                    }
                    if p.1 > 0 && field[p.0][p.1 - 1] == '.' {
                        v.push((p.0, p.1 - 1));
                    }
                    if p.0 < h - 1 && field[p.0 + 1][p.1] == '.' {
                        v.push((p.0 + 1, p.1));
                    }
                    if p.1 < w - 1 && field[p.0][p.1 + 1] == '.' {
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
        println!("{:?}", dmap);
        return 42;
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
    for line in lines.iter() {
        println!("{}", line);
    }
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
}
