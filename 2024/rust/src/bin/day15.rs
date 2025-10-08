use aoc2024::{Day, run_day};
use itertools::{Either, Itertools};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Box,
    BoxLeft,
    BoxRight,
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

struct Input {
    warehouse: Vec<Vec<Cell>>,
    movements: Vec<Move>,
    robot: (usize, usize),
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(String::from).collect_vec();
        let parts = lines
            .split(String::is_empty)
            .collect_tuple::<(_, _)>()
            .ok_or(Error::InvalidInput)?;
        let mut warehouse = parts
            .0
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
            movements: parts
                .1
                .join("")
                .chars()
                .map(Move::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

struct Solution;

impl Solution {
    fn sum_of_coordinates(input: &Input, double_width: bool) -> usize {
        let mut warehouse = input
            .warehouse
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|c| match c {
                        Cell::Box if double_width => {
                            Either::Left([Cell::BoxLeft, Cell::BoxRight].into_iter())
                        }
                        _ if double_width => Either::Left([*c, *c].into_iter()),
                        _ => Either::Right([*c].into_iter()),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let mut robot = (
            input.robot.0,
            input.robot.1 * (usize::from(double_width) + 1),
        );
        for movement in &input.movements {
            let cloned = warehouse.clone();
            if Self::try_move(robot, movement, &mut warehouse) {
                robot = movement.next(robot);
            } else {
                warehouse = cloned;
            }
        }
        warehouse
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, c)| match c {
                        Cell::Box | Cell::BoxLeft => Some(i * 100 + j),
                        _ => None,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
    fn try_move(src: (usize, usize), movement: &Move, warehouse: &mut [Vec<Cell>]) -> bool {
        let dst = movement.next(src);
        let can_move = match warehouse[dst.0][dst.1] {
            Cell::Box => Self::try_move(dst, movement, warehouse),
            Cell::BoxLeft => {
                Self::try_move((dst.0, dst.1 + 1), movement, warehouse)
                    && Self::try_move(dst, movement, warehouse)
            }
            Cell::BoxRight => {
                Self::try_move((dst.0, dst.1 - 1), movement, warehouse)
                    && Self::try_move(dst, movement, warehouse)
            }
            Cell::Wall => false,
            _ => true,
        };
        if can_move {
            warehouse[dst.0][dst.1] = warehouse[src.0][src.1];
            warehouse[src.0][src.1] = Cell::Empty;
        }
        can_move
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::sum_of_coordinates(input, false)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::sum_of_coordinates(input, true)
    }
}

fn main() -> Result<(), aoc2024::Error<Error>> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_small() -> Result<Input, Error> {
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
"
        .trim_start()
        .parse()
    }

    fn example_input() -> Result<Input, Error> {
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
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 10092);
        assert_eq!(Solution::part1(&example_input_small()?), 2028);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 9021);
        Ok(())
    }
}
