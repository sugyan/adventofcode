use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::stdin;

struct Solution {
    hs: HashMap<String, (u64, Vec<(u64, String)>)>,
}

impl Solution {
    fn solve1(&self) -> u64 {
        return self.need_for(1);
    }
    fn solve2(&self) -> u64 {
        let trillion = 1_000_000_000_000_u64;
        let (mut l, mut r) = (0, trillion);
        while r - l > 1 {
            let m = (l + r) / 2;
            match self.need_for(m).cmp(&trillion) {
                Ordering::Less => l = m,
                Ordering::Greater => r = m,
                Ordering::Equal => break,
            };
        }
        return l;
    }
    fn need_for(&self, target_amount: u64) -> u64 {
        let mut need: HashMap<String, u64> = HashMap::new();
        let mut surplus: HashMap<String, u64> = HashMap::new();
        need.insert("FUEL".to_string(), target_amount);
        loop {
            let target = if let Some(key) = need.keys().find(|p| self.hs[*p].1[0].1 != "ORE") {
                key.to_string()
            } else {
                break;
            };
            let mut amount = need[&target];
            if let Some(e) = surplus.get_mut(&target) {
                let d = std::cmp::min(amount, *e);
                amount -= d;
                *e -= d;
            }
            need.remove(&target);
            if amount > 0 {
                if let Some(reaction) = self.hs.get(&target) {
                    let n = (amount - 1) / reaction.0 + 1;
                    if reaction.0 * n > amount {
                        if let Some(e) = surplus.get_mut(&target) {
                            *e += reaction.0 * n - amount;
                        } else {
                            surplus.insert(target, reaction.0 * n - amount);
                        }
                    }
                    for material in reaction.1.iter() {
                        if let Some(amount) = need.get_mut(&material.1) {
                            *amount += material.0 * n;
                        } else {
                            need.insert(material.1.to_string(), material.0 * n);
                        }
                    }
                }
            }
        }
        let mut ret = 0;
        for (chemical, amount) in need {
            let n = (amount as u64 - 1) / self.hs[&chemical].0 + 1;
            ret += self.hs[&chemical].1[0].0 * n;
        }
        return ret;
    }
    fn new(inputs: Vec<String>) -> Self {
        let mut hs: HashMap<String, (u64, Vec<(u64, String)>)> = HashMap::new();
        for input in inputs.iter() {
            let parsed: Vec<Vec<Vec<&str>>> = input
                .trim()
                .split("=>")
                .map(|s| {
                    s.trim()
                        .split(',')
                        .map(|s| s.trim().split(' ').collect::<Vec<&str>>())
                        .collect::<Vec<Vec<&str>>>()
                })
                .collect();
            let amount = parsed[1][0][0].parse::<u64>().ok().unwrap();
            let target = parsed[1][0][1];
            let materials = parsed[0]
                .iter()
                .map(|e| (e[0].parse::<u64>().ok().unwrap(), e[1].to_string()))
                .collect();
            hs.insert(target.to_string(), (amount, materials));
        }
        return Solution { hs };
    }
}

fn main() {
    let mut inputs = Vec::new();
    loop {
        let mut buf = String::new();
        if let Ok(size) = stdin().read_line(&mut buf) {
            if size == 0 {
                break;
            }
        }
        inputs.push(buf.trim().to_string());
    }
    let solution = Solution::new(inputs);
    println!("{}", solution.solve1());
    println!("{}", solution.solve2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
";
        let solution = Solution::new(
            input
                .split('\n')
                .filter(|s| s.len() > 0)
                .map(|s| s.to_string())
                .collect(),
        );
        assert_eq!(31, solution.solve1());
    }

    #[test]
    fn example_2() {
        let input = "
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";
        let solution = Solution::new(
            input
                .split('\n')
                .filter(|s| s.len() > 0)
                .map(|s| s.to_string())
                .collect(),
        );
        assert_eq!(165, solution.solve1());
    }

    #[test]
    fn example_3() {
        let input = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
";
        let solution = Solution::new(
            input
                .split('\n')
                .filter(|s| s.len() > 0)
                .map(|s| s.to_string())
                .collect(),
        );
        assert_eq!(13312, solution.solve1());
        assert_eq!(82892753, solution.solve2());
    }

    #[test]
    fn example_4() {
        let input = "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
";
        let solution = Solution::new(
            input
                .split('\n')
                .filter(|s| s.len() > 0)
                .map(|s| s.to_string())
                .collect(),
        );
        assert_eq!(180697, solution.solve1());
        assert_eq!(5586022, solution.solve2());
    }

    #[test]
    fn example_5() {
        let input = "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
";
        let solution = Solution::new(
            input
                .split('\n')
                .filter(|s| s.len() > 0)
                .map(|s| s.to_string())
                .collect(),
        );
        assert_eq!(2210736, solution.solve1());
        assert_eq!(460664, solution.solve2());
    }
}
