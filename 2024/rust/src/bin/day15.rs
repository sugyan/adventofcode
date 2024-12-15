use aoc2024::{run, Solve};
use itertools::Itertools;
use std::{
    io::{BufRead, BufReader, Read},
    iter,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Box,
    Robot,
    Wall,
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            '#' => Ok(Self::Wall),
            _ => Err(Error::InvalidInput),
        }
    }
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn next(&self, (i, j): (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (i - 1, j),
            Self::Down => (i + 1, j),
            Self::Left => (i, j - 1),
            Self::Right => (i, j + 1),
        }
    }
}

impl TryFrom<char> for Move {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(Error::InvalidInput),
        }
    }
}

struct Solution {
    warehouse: Vec<Vec<Cell>>,
    movements: Vec<Move>,
    robot: (usize, usize),
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        BufReader::new(r)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .split(String::is_empty)
            .collect_tuple::<(_, _)>()
            .ok_or(Error::InvalidInput)
            .and_then(|(lines0, lines1)| {
                let mut warehouse = lines0
                    .iter()
                    .map(|line| {
                        line.chars()
                            .map(Cell::try_from)
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let robot = warehouse
                    .iter()
                    .enumerate()
                    .find_map(|(i, row)| {
                        row.iter().enumerate().find_map(|(j, c)| match c {
                            Cell::Robot => Some((i, j)),
                            _ => None,
                        })
                    })
                    .ok_or(Error::InvalidInput)?;
                warehouse[robot.0][robot.1] = Cell::Empty;
                Ok(Self {
                    warehouse,
                    robot,
                    movements: lines1
                        .join("")
                        .chars()
                        .map(Move::try_from)
                        .collect::<Result<_, _>>()?,
                })
            })
    }
    fn part1(&self) -> Self::Answer1 {
        let mut warehouse = self.warehouse.clone();
        let mut robot = self.robot;
        for movement in &self.movements {
            if let Some((i, j)) = iter::successors(Some(robot), |(i, j)| {
                Some(movement.next((*i, *j)))
                    .filter(|&(i, j)| !matches!(warehouse[i][j], Cell::Wall))
            })
            .skip(1)
            .find(|(i, j)| matches!(warehouse[*i][*j], Cell::Empty))
            {
                robot = movement.next(robot);
                warehouse[robot.0][robot.1] = Cell::Empty;
                if (i, j) != robot {
                    warehouse[i][j] = Cell::Box;
                }
            }
        }
        warehouse
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, c)| match c {
                        Cell::Box => Some(i * 100 + j),
                        _ => None,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_small() -> &'static [u8] {
        r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"[1..]
            .as_bytes()
    }

    fn example_input() -> &'static [u8] {
        r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 10092);
        assert_eq!(Solution::new(example_input_small())?.part1(), 2028);
        Ok(())
    }
}
