use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::str::FromStr;

struct Game {
    id: u32,
    subsets: Vec<Cubes>,
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(": ").unwrap();
        let id = l[5..].parse()?;
        let subsets = r.split("; ").filter_map(|s| s.parse().ok()).collect();
        Ok(Self { id, subsets })
    }
}

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Cubes {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for part in s.split(", ") {
            let (n, color) = part.split_once(' ').unwrap();
            let n = n.parse()?;
            match color {
                "red" => red = n,
                "green" => green = n,
                "blue" => blue = n,
                _ => unreachable!(),
            }
        }
        Ok(Self { red, green, blue })
    }
}

struct Solution {
    games: Vec<Game>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let games = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .map(|s| s.parse().unwrap())
            .collect();
        Self { games }
    }
    fn part1(&self) -> Self::Answer1 {
        self.games
            .iter()
            .filter_map(|game| {
                if game
                    .subsets
                    .iter()
                    .all(|subset| subset.red <= 12 && subset.green <= 13 && subset.blue <= 14)
                {
                    Some(game.id)
                } else {
                    None
                }
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.games
            .iter()
            .map(|game| {
                let (r, g, b) = game.subsets.iter().fold((0, 0, 0), |(r, g, b), cubes| {
                    (r.max(cubes.red), g.max(cubes.green), b.max(cubes.blue))
                });
                r * g * b
            })
            .sum()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 2286);
    }
}
