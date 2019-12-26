use std::collections::{HashMap, HashSet};
use std::io::stdin;

struct Solution {
    tiles: [[char; 5]; 5],
}

impl Solution {
    pub fn new(tiles: [[char; 5]; 5]) -> Solution {
        return Solution { tiles };
    }
    fn solve1(&self) -> i32 {
        let mut tiles = self.tiles;
        let mut hs: HashSet<i32> = HashSet::new();
        hs.insert(self.hash(tiles));
        let mut next = [['\0'; 5]; 5];
        loop {
            for i in 0..5 {
                for j in 0..5 {
                    let mut count = 0;
                    if i > 0 && tiles[i - 1][j] == '#' {
                        count += 1
                    }
                    if j > 0 && tiles[i][j - 1] == '#' {
                        count += 1
                    }
                    if i < 4 && tiles[i + 1][j] == '#' {
                        count += 1
                    }
                    if j < 4 && tiles[i][j + 1] == '#' {
                        count += 1
                    }
                    match tiles[i][j] {
                        '#' => next[i][j] = if count == 1 { '#' } else { '.' },
                        '.' => next[i][j] = if count == 1 || count == 2 { '#' } else { '.' },
                        _ => println!("invalid tile: {}", tiles[i][j]),
                    }
                }
            }
            tiles = next;
            let hash = self.hash(tiles);
            if hs.contains(&hash) {
                return hash;
            } else {
                hs.insert(hash);
            }
        }
    }
    fn solve2(&self, minutes: usize) -> i32 {
        let mut hm: HashMap<i32, ([[char; 5]; 5], [[char; 5]; 5])> = HashMap::new();
        let mut tiles = self.tiles;
        tiles[2][2] = '?';
        hm.insert(0, (tiles, tiles));
        for _ in 0..minutes {
            let mut keys: Vec<i32> = hm.keys().map(|k| *k).collect();
            keys.sort();
            for key in keys.iter() {
                let mut next = [0; 4];
                let mut prev = [0; 4];
                if let Some(tiles) = hm.get(&(key + 1)) {
                    for i in 0..5 {
                        if tiles.0[i][0] == '#' {
                            next[0] += 1;
                        }
                        if tiles.0[0][i] == '#' {
                            next[1] += 1;
                        }
                        if tiles.0[i][4] == '#' {
                            next[2] += 1;
                        }
                        if tiles.0[4][i] == '#' {
                            next[3] += 1;
                        }
                    }
                }
                if let Some(tiles) = hm.get(&(key - 1)) {
                    if tiles.0[2][1] == '#' {
                        prev[0] += 1;
                    }
                    if tiles.0[1][2] == '#' {
                        prev[1] += 1;
                    }
                    if tiles.0[2][3] == '#' {
                        prev[2] += 1;
                    }
                    if tiles.0[3][2] == '#' {
                        prev[3] += 1;
                    }
                }
                let mut tiles = hm.get_mut(&key).unwrap();
                for i in 0..5 {
                    for j in 0..5 {
                        let mut count = 0;
                        if i > 0 {
                            match tiles.0[i - 1][j] {
                                '#' => count += 1,
                                '?' => count += next[3],
                                _ => {}
                            }
                        } else {
                            count += prev[1];
                        }
                        if j > 0 {
                            match tiles.0[i][j - 1] {
                                '#' => count += 1,
                                '?' => count += next[2],
                                _ => {}
                            }
                        } else {
                            count += prev[0];
                        }
                        if i < 4 {
                            match tiles.0[i + 1][j] {
                                '#' => count += 1,
                                '?' => count += next[1],
                                _ => {}
                            }
                        } else {
                            count += prev[3]
                        }
                        if j < 4 {
                            match tiles.0[i][j + 1] {
                                '#' => count += 1,
                                '?' => count += next[0],
                                _ => {}
                            }
                        } else {
                            count += prev[2]
                        }
                        tiles.1[i][j] = match tiles.0[i][j] {
                            '#' => {
                                if count == 1 {
                                    '#'
                                } else {
                                    '.'
                                }
                            }
                            '.' => {
                                if count == 1 || count == 2 {
                                    '#'
                                } else {
                                    '.'
                                }
                            }
                            '?' => tiles.0[i][j],
                            _ => {
                                println!("invalid tile: {}", tiles.0[i][j]);
                                '\0'
                            }
                        }
                    }
                }
                if let Some(min) = keys.first() {
                    let tiles = hm.get_mut(&min).unwrap();
                    let mut next = [['.'; 5]; 5];
                    let counts = vec![
                        (0..5).filter(|i| tiles.0[*i][0] == '#').count(),
                        (0..5).filter(|i| tiles.0[0][*i] == '#').count(),
                        (0..5).filter(|i| tiles.0[*i][4] == '#').count(),
                        (0..5).filter(|i| tiles.0[4][*i] == '#').count(),
                    ];
                    if counts[0] == 1 || counts[0] == 2 {
                        next[2][1] = '#';
                    }
                    if counts[1] == 1 || counts[1] == 2 {
                        next[1][2] = '#';
                    }
                    if counts[2] == 1 || counts[2] == 2 {
                        next[2][3] = '#';
                    }
                    if counts[3] == 1 || counts[3] == 2 {
                        next[3][2] = '#';
                    }
                    if next[2][1] == '#'
                        || next[1][2] == '#'
                        || next[2][3] == '#'
                        || next[3][2] == '#'
                    {
                        next[2][2] = '?';
                        hm.insert(min - 1, ([['\0'; 5]; 5], next));
                    }
                }
                if let Some(max) = keys.last() {
                    let tiles = hm.get_mut(&max).unwrap();
                    if tiles.0[2][1] == '#'
                        || tiles.0[1][2] == '#'
                        || tiles.0[2][3] == '#'
                        || tiles.0[3][2] == '#'
                    {
                        let mut next = [['.'; 5]; 5];
                        next[2][2] = '?';
                        if tiles.0[2][1] == '#' {
                            for i in 0..5 {
                                next[i][0] = '#';
                            }
                        }
                        if tiles.0[1][2] == '#' {
                            for i in 0..5 {
                                next[0][i] = '#';
                            }
                        }
                        if tiles.0[2][3] == '#' {
                            for i in 0..5 {
                                next[i][4] = '#';
                            }
                        }
                        if tiles.0[3][2] == '#' {
                            for i in 0..5 {
                                next[4][i] = '#';
                            }
                        }
                        hm.insert(max + 1, ([['\0'; 5]; 5], next));
                    }
                }
            }
            for tiles in hm.values_mut() {
                tiles.0 = tiles.1
            }
        }
        let mut answer = 0;
        for tiles in hm.values() {
            for i in 0..5 {
                for j in 0..5 {
                    if tiles.0[i][j] == '#' {
                        answer += 1;
                    }
                }
            }
        }
        return answer;
    }
    fn hash(&self, tiles: [[char; 5]; 5]) -> i32 {
        let mut ret = 0;
        for i in 0..5 {
            for j in 0..5 {
                if tiles[i][j] == '#' {
                    ret += 1 << (i * 5 + j);
                }
            }
        }
        return ret;
    }
}

fn main() {
    let mut tiles = [['\0'; 5]; 5];
    for i in 0..5 {
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        for (j, c) in (0..5).zip(buf.chars()) {
            tiles[i][j] = c;
        }
    }
    let solution = Solution::new(tiles);
    println!("{}", solution.solve1());
    println!("{}", solution.solve2(200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let tiles = [
            ['.', '.', '.', '.', '#'],
            ['#', '.', '.', '#', '.'],
            ['#', '.', '.', '#', '#'],
            ['.', '.', '#', '.', '.'],
            ['#', '.', '.', '.', '.'],
        ];
        assert_eq!(2129920, Solution::new(tiles).solve1())
    }

    #[test]
    fn example_2() {
        let tiles = [
            ['.', '.', '.', '.', '#'],
            ['#', '.', '.', '#', '.'],
            ['#', '.', '?', '#', '#'],
            ['.', '.', '#', '.', '.'],
            ['#', '.', '.', '.', '.'],
        ];
        assert_eq!(99, Solution::new(tiles).solve2(10))
    }
}
