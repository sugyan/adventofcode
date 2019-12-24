use std::collections::HashSet;
use std::io::stdin;

struct Solution {
    tiles: [[char; 5]; 5],
}

impl Solution {
    pub fn new(tiles: [[char; 5]; 5]) -> Solution {
        return Solution { tiles };
    }
    fn solve1(&mut self) -> i32 {
        let mut hs: HashSet<i32> = HashSet::new();
        hs.insert(self.hash());
        let mut tiles = [['\0'; 5]; 5];
        loop {
            for i in 0..5 {
                for j in 0..5 {
                    let mut count = 0;
                    if i > 0 && self.tiles[i - 1][j] == '#' {
                        count += 1
                    }
                    if j > 0 && self.tiles[i][j - 1] == '#' {
                        count += 1
                    }
                    if i < 4 && self.tiles[i + 1][j] == '#' {
                        count += 1
                    }
                    if j < 4 && self.tiles[i][j + 1] == '#' {
                        count += 1
                    }
                    match self.tiles[i][j] {
                        '#' => tiles[i][j] = if count == 1 { '#' } else { '.' },
                        '.' => tiles[i][j] = if count == 1 || count == 2 { '#' } else { '.' },
                        _ => println!("invalid tile: {}", self.tiles[i][j]),
                    }
                }
            }
            self.tiles = tiles;
            let hash = self.hash();
            if hs.contains(&hash) {
                return hash;
            } else {
                hs.insert(hash);
            }
        }
    }
    fn hash(&self) -> i32 {
        let mut ret = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.tiles[i][j] == '#' {
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
    let mut solution = Solution::new(tiles);
    println!("{}", solution.solve1());
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
}
