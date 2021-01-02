use std::collections::HashMap;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
enum Orientation {
    Rotate000,
    Rotate090,
    Rotate180,
    Rotate270,
    Rotate000Flipped,
    Rotate090Flipped,
    Rotate180Flipped,
    Rotate270Flipped,
}

impl Orientation {
    fn all() -> [Orientation; 8] {
        [
            Orientation::Rotate000,
            Orientation::Rotate090,
            Orientation::Rotate180,
            Orientation::Rotate270,
            Orientation::Rotate000Flipped,
            Orientation::Rotate090Flipped,
            Orientation::Rotate180Flipped,
            Orientation::Rotate270Flipped,
        ]
    }
}

#[derive(Clone)]
struct Tile {
    image: Vec<Vec<bool>>,
}

struct Borders {
    top: Vec<bool>,
    left: Vec<bool>,
    bottom: Vec<bool>,
    right: Vec<bool>,
}

impl Borders {
    fn all(&self) -> [&Vec<bool>; 4] {
        [&self.top, &self.left, &self.bottom, &self.right]
    }
}

impl Tile {
    fn borders(&self, orientation: Orientation) -> Borders {
        let size = self.image.len();
        match orientation {
            Orientation::Rotate000 => Borders {
                top: self.image[0].clone(),
                left: self.image.iter().map(|row| row[0]).collect(),
                bottom: self.image[size - 1].clone(),
                right: self.image.iter().map(|row| row[row.len() - 1]).collect(),
            },
            Orientation::Rotate090 => Borders {
                top: self.image.iter().map(|row| row[0]).rev().collect(),
                left: self.image[size - 1].clone(),
                bottom: self
                    .image
                    .iter()
                    .map(|row| row[row.len() - 1])
                    .rev()
                    .collect(),
                right: self.image[0].clone(),
            },
            Orientation::Rotate180 => Borders {
                top: self.image[size - 1].clone().into_iter().rev().collect(),
                left: self
                    .image
                    .iter()
                    .map(|row| row[row.len() - 1])
                    .rev()
                    .collect(),
                bottom: self.image[0].clone().into_iter().rev().collect(),
                right: self.image.iter().map(|row| row[0]).rev().collect(),
            },
            Orientation::Rotate270 => Borders {
                top: self.image.iter().map(|row| row[row.len() - 1]).collect(),
                left: self.image[0].clone().into_iter().rev().collect(),
                bottom: self.image.iter().map(|row| row[0]).collect(),
                right: self.image[size - 1].clone().into_iter().rev().collect(),
            },
            Orientation::Rotate000Flipped => Borders {
                top: self.image.iter().map(|row| row[0]).collect(),
                left: self.image[0].clone(),
                bottom: self.image.iter().map(|row| row[row.len() - 1]).collect(),
                right: self.image[size - 1].clone(),
            },
            Orientation::Rotate090Flipped => Borders {
                top: self.image[size - 1].clone(),
                left: self.image.iter().map(|row| row[0]).rev().collect(),
                bottom: self.image[0].clone(),
                right: self
                    .image
                    .iter()
                    .map(|row| row[row.len() - 1])
                    .rev()
                    .collect(),
            },
            Orientation::Rotate180Flipped => Borders {
                top: self
                    .image
                    .iter()
                    .map(|row| row[row.len() - 1])
                    .rev()
                    .collect(),
                left: self.image[size - 1].clone().into_iter().rev().collect(),
                bottom: self.image.iter().map(|row| row[0]).rev().collect(),
                right: self.image[0].clone().into_iter().rev().collect(),
            },
            Orientation::Rotate270Flipped => Borders {
                top: self.image[0].clone().into_iter().rev().collect(),
                left: self.image.iter().map(|row| row[row.len() - 1]).collect(),
                bottom: self.image[size - 1].clone().into_iter().rev().collect(),
                right: self.image.iter().map(|row| row[0]).collect(),
            },
        }
    }
    fn translated(&self, orientation: Orientation) -> Vec<Vec<bool>> {
        let size = self.image.len();
        (0..size)
            .map(|i| {
                (0..size)
                    .map(|j| match orientation {
                        Orientation::Rotate000 => self.image[i][j],
                        Orientation::Rotate090 => self.image[size - 1 - j][i],
                        Orientation::Rotate180 => self.image[size - 1 - i][size - 1 - j],
                        Orientation::Rotate270 => self.image[j][size - 1 - i],
                        Orientation::Rotate000Flipped => self.image[j][i],
                        Orientation::Rotate090Flipped => self.image[size - 1 - i][j],
                        Orientation::Rotate180Flipped => self.image[size - 1 - j][size - 1 - i],
                        Orientation::Rotate270Flipped => self.image[i][size - 1 - j],
                    })
                    .collect()
            })
            .collect()
    }
}

struct Solution {
    tiles: Vec<Vec<(u64, Tile, Orientation)>>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let mut tiles_map = HashMap::new();
        let (mut id, mut tile) = (0, Vec::new());
        for line in inputs.iter() {
            if let Some(idstr) = line.strip_prefix("Tile ").map(|s| s.trim_end_matches(':')) {
                if let Ok(n) = idstr.parse::<u64>() {
                    id = n;
                }
            } else if line.is_empty() {
                tiles_map.insert(
                    id,
                    Tile {
                        image: tile.clone(),
                    },
                );
                tile.clear();
            } else {
                tile.push(line.chars().map(|c| c == '#').collect());
            }
        }
        if !tile.is_empty() {
            tiles_map.insert(id, Tile { image: tile });
        }
        Self {
            tiles: Solution::build_image(&tiles_map),
        }
    }
    fn solve_1(&self) -> u64 {
        let (row, col) = (self.tiles.len(), self.tiles[0].len());
        [
            self.tiles[0][0].0,
            self.tiles[0][col - 1].0,
            self.tiles[row - 1][0].0,
            self.tiles[row - 1][col - 1].0,
        ]
        .iter()
        .product()
    }
    fn solve_2(&self) -> usize {
        let actual = Tile {
            image: self
                .tiles
                .iter()
                .map(|row| {
                    let size = row[0].1.image.len();
                    (1..size - 1)
                        .map(|i| {
                            row.iter()
                                .map(|tile| {
                                    tile.1.translated(tile.2)[i]
                                        .clone()
                                        .into_iter()
                                        .skip(1)
                                        .take(size - 2)
                                        .collect::<Vec<bool>>()
                                })
                                .flatten()
                                .collect()
                        })
                        .collect::<Vec<Vec<bool>>>()
                })
                .flatten()
                .collect(),
        };
        let sea_monster = [
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ]
        .iter()
        .enumerate()
        .map(|(i, &row)| {
            row.chars()
                .enumerate()
                .filter_map(|(j, c)| if c == '#' { Some((i, j)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>();
        let mut ret = actual
            .image
            .iter()
            .map(|row| row.iter().filter(|&b| *b).count())
            .sum();
        if let Some(found) = Orientation::all()
            .iter()
            .map(|&orientation| {
                let image = actual.translated(orientation);
                let mut count = 0;
                for i in 0..image.len() {
                    for j in 0..image[i].len() {
                        if sea_monster.iter().all(|&(di, dj)| {
                            i + di < image.len() && j + dj < image[i].len() && image[i + di][j + dj]
                        }) {
                            count += 1;
                        }
                    }
                }
                count
            })
            .max()
        {
            ret -= found * sea_monster.len()
        }
        ret
    }
    fn build_image(tiles_map: &HashMap<u64, Tile>) -> Vec<Vec<(u64, Tile, Orientation)>> {
        let mut borders_map = HashMap::new();
        for (&id, tile) in tiles_map.iter() {
            for &orientation in [Orientation::Rotate000, Orientation::Rotate180].iter() {
                for &border in tile.borders(orientation).all().iter() {
                    borders_map
                        .entry(border.clone())
                        .or_insert_with(Vec::new)
                        .push(id);
                }
            }
        }
        let mut tiles = Vec::new();
        if let Some((&id, tile)) = tiles_map.iter().find(|&(_, tile)| {
            tile.borders(Orientation::Rotate000)
                .all()
                .iter()
                .filter_map(|&border| borders_map.get(border))
                .filter(|&v| v.len() == 1)
                .count()
                == 2
        }) {
            if let Some(&orientation) = Orientation::all().iter().find(|&orientation| {
                tile.borders(*orientation)
                    .all()
                    .iter()
                    .filter_map(|&border| borders_map.get(border))
                    .map(|v| v.len())
                    .collect::<Vec<usize>>()
                    == vec![1, 1, 2, 2]
            }) {
                let mut row = vec![(id, tile.clone(), orientation)];
                while let Some(last) = row.last() {
                    let right = last.1.borders(last.2).right;
                    if let Some(ids) = borders_map.get(&right) {
                        if ids.len() != 2 {
                            break;
                        }
                        let id = if ids[0] == last.0 { ids[1] } else { ids[0] };
                        if let Some(tile) = tiles_map.get(&id) {
                            if let Some(&orientation) = Orientation::all()
                                .iter()
                                .find(|&orientation| tile.borders(*orientation).left == right)
                            {
                                row.push((id, tile.clone(), orientation));
                            }
                        }
                    }
                }
                tiles.push(row);
            }
        }
        while let Some(last_row) = tiles.last() {
            let mut row = Vec::with_capacity(last_row.len());
            for last in last_row.iter() {
                let bottom = last.1.borders(last.2).bottom;
                if let Some(ids) = borders_map.get(&bottom) {
                    if ids.len() != 2 {
                        break;
                    }
                    let id = if ids[0] == last.0 { ids[1] } else { ids[0] };
                    if let Some(tile) = tiles_map.get(&id) {
                        if let Some(&orientation) = Orientation::all()
                            .iter()
                            .find(|&orientation| tile.borders(*orientation).top == bottom)
                        {
                            row.push((id, tile.clone(), orientation));
                        }
                    }
                }
            }
            if row.is_empty() {
                break;
            } else {
                tiles.push(row);
            }
        }
        tiles
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
    println!("{}", solution.solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solution() -> Solution {
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
                .collect(),
        )
    }

    #[test]
    fn example_1() {
        assert_eq!(20_899_048_083_289, solution().solve_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(273, solution().solve_2());
    }
}
