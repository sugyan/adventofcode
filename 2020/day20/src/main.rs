use std::collections::HashMap;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
enum Orientation {
    Rotate000,
    Rotate090,
    Rotate180,
    Rotate270,
    Mirror000,
    Mirror090,
    Mirror180,
    Mirror270,
}

struct Tile {
    id: u64,
    orientation: Orientation,
    image: Image,
}

#[derive(Clone)]
struct Image {
    data: Vec<Vec<char>>,
}

struct Borders {
    top: String,
    bottom: String,
    left: String,
    right: String,
}

struct Solution {
    tiles: Vec<Vec<Tile>>,
}

impl Orientation {
    fn all() -> [Orientation; 8] {
        [
            Orientation::Rotate000,
            Orientation::Rotate090,
            Orientation::Rotate180,
            Orientation::Rotate270,
            Orientation::Mirror000,
            Orientation::Mirror090,
            Orientation::Mirror180,
            Orientation::Mirror270,
        ]
    }
}

impl Borders {
    fn all(&self) -> [&String; 4] {
        [&self.top, &self.bottom, &self.left, &self.right]
    }
}

impl Image {
    fn borders(&self, orientation: Orientation) -> Borders {
        let len = self.data.len();
        match orientation {
            Orientation::Rotate000 => Borders {
                top: self.data[0].iter().collect(),
                bottom: self.data[len - 1].iter().collect(),
                left: (0..len).map(|i| self.data[i][0]).collect(),
                right: (0..len).map(|i| self.data[i][len - 1]).collect(),
            },
            Orientation::Rotate090 => Borders {
                top: (0..len).map(|i| self.data[len - 1 - i][0]).collect(),
                bottom: (0..len).map(|i| self.data[len - 1 - i][len - 1]).collect(),
                left: self.data[len - 1].iter().collect(),
                right: self.data[0].iter().collect(),
            },
            Orientation::Rotate180 => Borders {
                top: self.data[len - 1].iter().rev().collect(),
                bottom: self.data[0].iter().rev().collect(),
                left: (0..len).map(|i| self.data[len - 1 - i][len - 1]).collect(),
                right: (0..len).map(|i| self.data[len - 1 - i][0]).collect(),
            },
            Orientation::Rotate270 => Borders {
                top: (0..len).map(|i| self.data[i][len - 1]).collect(),
                bottom: (0..len).map(|i| self.data[i][0]).collect(),
                left: self.data[0].iter().rev().collect(),
                right: self.data[len - 1].iter().rev().collect(),
            },
            Orientation::Mirror000 => Borders {
                top: (0..len).map(|i| self.data[i][0]).collect(),
                bottom: (0..len).map(|i| self.data[i][len - 1]).collect(),
                left: self.data[0].iter().collect(),
                right: self.data[len - 1].iter().collect(),
            },
            Orientation::Mirror090 => Borders {
                top: self.data[0].iter().rev().collect(),
                bottom: self.data[len - 1].iter().rev().collect(),
                left: (0..len).map(|i| self.data[i][len - 1]).collect(),
                right: (0..len).map(|i| self.data[i][0]).collect(),
            },
            Orientation::Mirror180 => Borders {
                top: (0..len).map(|i| self.data[len - 1 - i][len - 1]).collect(),
                bottom: (0..len).map(|i| self.data[len - 1 - i][0]).collect(),
                left: self.data[len - 1].iter().rev().collect(),
                right: self.data[0].iter().rev().collect(),
            },
            Orientation::Mirror270 => Borders {
                top: self.data[len - 1].iter().collect(),
                bottom: self.data[0].iter().collect(),
                left: (0..len).map(|i| self.data[len - 1 - i][0]).collect(),
                right: (0..len).map(|i| self.data[len - 1 - i][len - 1]).collect(),
            },
        }
    }
    fn rotated(&self, orientation: Orientation) -> Vec<Vec<char>> {
        let len = self.data.len();
        (0..len)
            .map(|i| {
                (0..len)
                    .map(|j| match orientation {
                        Orientation::Rotate000 => self.data[i][j],
                        Orientation::Rotate090 => self.data[len - 1 - j][i],
                        Orientation::Rotate180 => self.data[len - 1 - i][len - 1 - j],
                        Orientation::Rotate270 => self.data[j][len - 1 - i],
                        Orientation::Mirror000 => self.data[j][i],
                        Orientation::Mirror090 => self.data[i][len - 1 - j],
                        Orientation::Mirror180 => self.data[len - 1 - j][len - 1 - i],
                        Orientation::Mirror270 => self.data[len - 1 - i][j],
                    })
                    .collect()
            })
            .collect()
    }
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let mut imagemap: HashMap<u64, Image> = HashMap::new();
        {
            let mut id: u64 = 0;
            let mut tile: Vec<Vec<char>> = Vec::new();
            for line in inputs.iter() {
                if let Some(idstr) = line.strip_prefix("Tile ").map(|s| s.trim_end_matches(':')) {
                    if let Ok(n) = idstr.parse::<u64>() {
                        id = n;
                    }
                } else if line.is_empty() {
                    imagemap.insert(id, Image { data: tile.clone() });
                    tile.clear();
                } else {
                    tile.push(line.chars().collect());
                }
            }
            if !tile.is_empty() {
                imagemap.insert(id, Image { data: tile });
            }
        }
        let mut edgesdict: HashMap<String, Vec<u64>> = HashMap::new();
        for (&k, image) in imagemap.iter() {
            for &border in image.borders(Orientation::Rotate000).all().iter() {
                edgesdict
                    .entry(border.clone())
                    .or_insert_with(Vec::new)
                    .push(k);
            }
            for &border in image.borders(Orientation::Mirror180).all().iter() {
                edgesdict
                    .entry(border.clone())
                    .or_insert_with(Vec::new)
                    .push(k);
            }
        }
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        if let Some((&id, image)) = imagemap.iter().find(|(_, tile)| {
            tile.borders(Orientation::Rotate000)
                .all()
                .iter()
                .filter(|&&border| {
                    if let Some(v) = edgesdict.get(border) {
                        v.len() == 1
                    } else {
                        false
                    }
                })
                .count()
                == 2
        }) {
            let mut row: Vec<Tile> = Vec::new();
            if let Some(&orientation) = Orientation::all().iter().find(|&orientation| {
                image
                    .borders(*orientation)
                    .all()
                    .iter()
                    .filter_map(|&border| edgesdict.get(border))
                    .map(|v| v.len())
                    .collect::<Vec<usize>>()
                    == vec![1, 2, 1, 2]
            }) {
                row.push(Tile {
                    id,
                    orientation,
                    image: image.clone(),
                })
            }
            'row: loop {
                if let Some(last) = row.last() {
                    let border = &last.image.borders(last.orientation).right;
                    if let Some(ids) = edgesdict.get(border) {
                        if let Some(next) = ids.iter().find(|&id| *id != last.id) {
                            if let Some(image) = imagemap.get(next) {
                                if let Some(&orientation) =
                                    Orientation::all().iter().find(|&orientation| {
                                        image.borders(*orientation).left == *border
                                    })
                                {
                                    row.push(Tile {
                                        id: *next,
                                        orientation,
                                        image: image.clone(),
                                    });
                                }
                            }
                        } else {
                            break 'row;
                        }
                    }
                }
            }
            tiles.push(row);
        }
        for i in 1..tiles[0].len() {
            let mut row: Vec<Tile> = Vec::new();
            for col in tiles[i - 1].iter() {
                let border = &col.image.borders(col.orientation).bottom;
                if let Some(ids) = edgesdict.get(border) {
                    if let Some(next) = ids.iter().find(|&id| *id != col.id) {
                        if let Some(image) = imagemap.get(next) {
                            if let Some(&orientation) = Orientation::all()
                                .iter()
                                .find(|&orientation| image.borders(*orientation).top == *border)
                            {
                                row.push(Tile {
                                    id: *next,
                                    orientation,
                                    image: image.clone(),
                                });
                            }
                        }
                    }
                }
            }
            tiles.push(row);
        }
        Self { tiles }
    }
    fn solve_1(&self) -> u64 {
        let size = self.tiles.len();
        [
            &self.tiles[0][0],
            &self.tiles[0][size - 1],
            &self.tiles[size - 1][0],
            &self.tiles[size - 1][size - 1],
        ]
        .iter()
        .map(|&image| image.id)
        .product()
    }
    fn solve_2(&self) -> u64 {
        let size = self.tiles.len();
        let tile_size = self.tiles[0][0].image.data.len();
        let mut image = Image {
            data: Vec::with_capacity((tile_size - 2) * size),
        };
        for i in 0..size {
            let images: Vec<Vec<Vec<char>>> = self.tiles[i]
                .iter()
                .map(|tile| tile.image.rotated(tile.orientation))
                .collect();
            for j in 1..tile_size - 1 {
                let mut row: Vec<char> = Vec::with_capacity((tile_size - 2) * size);
                for image in images.iter() {
                    row.extend(image[j].iter().skip(1).take(tile_size - 2));
                }
                image.data.push(row);
            }
        }
        let sea_monster: Vec<Vec<char>> = [
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ]
        .iter()
        .map(|s| s.chars().collect())
        .collect();
        let positions: Vec<(usize, usize)> = sea_monster
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &col)| col == '#')
                    .map(|(j, _)| (i, j))
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect();
        if let Some(found) = Orientation::all()
            .iter()
            .map(|&orientation| {
                let rotated = image.rotated(orientation);
                let mut count = 0;
                for i in 0..rotated.len() - sea_monster.len() {
                    for j in 0..rotated.len() - sea_monster[0].len() {
                        if positions.iter().all(|&p| rotated[i + p.0][j + p.1] == '#') {
                            count += 1;
                        }
                    }
                }
                count
            })
            .max()
        {
            (image
                .data
                .iter()
                .map(|row| row.iter().filter(|&c| *c == '#').count())
                .sum::<usize>()
                - positions.len() * found) as u64
        } else {
            0
        }
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
