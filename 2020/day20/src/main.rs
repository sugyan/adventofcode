use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> u64 {
        let mut tiles: HashMap<u64, Vec<Vec<bool>>> = HashMap::new();
        let mut id: u64 = 0;
        let mut tile: Vec<Vec<bool>> = Vec::new();
        for line in self.inputs.iter() {
            if let Some(idstr) = line.strip_prefix("Tile ").map(|s| s.trim_end_matches(':')) {
                if let Ok(n) = idstr.parse::<u64>() {
                    id = n;
                }
            } else if line.is_empty() {
                tiles.insert(id, tile.clone());
                tile.clear();
            } else {
                tile.push(line.chars().map(|c| c == '#').collect());
            }
        }
        if !tile.is_empty() {
            tiles.insert(id, tile);
        }
        let mut edges: HashMap<u64, Vec<u32>> = HashMap::new();
        for (&k, v) in tiles.iter() {
            let size = v.len();
            let values: Vec<Vec<bool>> = (0..size)
                .map(|i| {
                    vec![
                        v[0][i],
                        v[0][size - 1 - i],
                        v[size - 1][i],
                        v[size - 1][size - 1 - i],
                        v[i][0],
                        v[size - 1 - i][0],
                        v[i][size - 1],
                        v[size - 1 - i][size - 1],
                    ]
                })
                .collect();
            edges.insert(
                k,
                (0..8)
                    .map(|i| {
                        (0..size)
                            .map(|j| if values[j][i] { 1 << j } else { 0 })
                            .sum()
                    })
                    .collect(),
            );
        }
        let mut hm: HashMap<u32, usize> = HashMap::new();
        for v in edges.values() {
            v.iter().for_each(|&val| *hm.entry(val).or_insert(0) += 1);
        }
        edges
            .iter()
            .filter(|(_, v)| {
                v.iter()
                    .filter(|&val| {
                        if let Some(&n) = hm.get(val) {
                            n == 2
                        } else {
                            false
                        }
                    })
                    .count()
                    == 4
            })
            .map(|(&k, _)| k)
            .product()
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            20_899_048_083_289,
            Solution::new(
                "
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
