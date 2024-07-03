use std::{collections::HashMap, fmt::Display};

use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn path(input: &str) -> IResult<&str, Vec<IVec2>> {
    separated_list1(
        tag(" -> "),
        separated_pair(complete::i32, tag(","), complete::i32)
            .map(|point| IVec2::new(point.0, point.1)),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<IVec2>>> {
    separated_list1(newline, path)(input)
}

#[derive(Debug, PartialEq)]
enum Block {
    Stone,
    Sand,
}

enum Direction {
    Down,
    DownLeft,
    DownRight,
}

struct Grid {
    grid: HashMap<IVec2, Block>,
    floor_y: i32,
}

impl Grid {
    fn new(path_points: Vec<Vec<IVec2>>) -> Self {
        let mut grid = HashMap::new();

        for points in path_points {
            for (first, second) in points.iter().tuple_windows() {
                let x_min = first.x.min(second.x);
                let x_max = first.x.max(second.x);
                let y_min = first.y.min(second.y);
                let y_max = first.y.max(second.y);

                // dbg!(&x_min, &x_max, &y_min, &y_max);

                let x_range = x_min..=x_max;
                let y_range = y_min..=y_max;

                x_range.cartesian_product(y_range).for_each(|(x, y)| {
                    grid.insert(IVec2::new(x, y), Block::Stone);
                })
            }
        }

        // dbg!(&grid);

        let floor_y = grid.keys().map(|point| point.y).max().unwrap() + 2;

        Self { grid, floor_y }
    }

    fn can_go(&self, sand: IVec2, direction: Direction) -> bool {
        match direction {
            Direction::Down => !self.grid.contains_key(&(sand + IVec2::Y)),
            Direction::DownLeft => !self.grid.contains_key(&(sand + IVec2::new(-1, 1))),
            Direction::DownRight => !self.grid.contains_key(&(sand + IVec2::new(1, 1))),
        }
    }

    fn drip(&mut self) -> Option<IVec2> {
        let mut sand = IVec2::new(500, 0);

        let lowest_y = self.grid.keys().map(|position| position.y).max().unwrap();

        loop {
            if self.can_go(sand, Direction::Down) {
                sand += IVec2::Y;
            } else if self.can_go(sand, Direction::DownLeft) {
                sand += IVec2::new(-1, 1);
            } else if self.can_go(sand, Direction::DownRight) {
                sand += IVec2::new(1, 1);
            } else {
                self.grid.insert(sand, Block::Sand);
                if sand == IVec2::new(500, 0) {
                    return None;
                } else {
                    return Some(sand);
                }
            }

            if sand.y == self.floor_y - 1 {
                let settled_sand = IVec2::new(sand.x, self.floor_y - 1);
                self.grid.insert(settled_sand, Block::Sand);
                return Some(settled_sand);
            }
        }
    }

    fn sand_amt(&self) -> usize {
        self.grid
            .values()
            .filter(|block| **block == Block::Sand)
            .count()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.grid.keys().map(|point| point.x).min().unwrap();
        let x_max = self.grid.keys().map(|point| point.x).max().unwrap();
        let y_min = self.grid.keys().map(|point| point.y).min().unwrap();
        let y_max = self.grid.keys().map(|point| point.y).max().unwrap();

        // dbg!(&x_min, &x_max, &y_min, &y_max);

        let mut output = String::new();

        for y in 0..=y_max {
            for x in x_min..=x_max {
                let ch = if x == 500 && y == 0 {
                    '+'
                } else if let Some(block) = self.grid.get(&IVec2::new(x, y)) {
                    match block {
                        Block::Stone => '#',
                        Block::Sand => 'o',
                    }
                } else {
                    '.'
                };

                output.push(ch);
            }
            output.push('\n');
        }

        writeln!(f, "{output}")
    }
}

fn process(input: &str) -> String {
    let (input, path_points) = parse(input).expect("should parse");

    // dbg!(&input, &path_points);

    let mut grid = Grid::new(path_points);

    println!("{grid}");

    while grid.drip().is_some() {
        println!("{grid}");
        dbg!(grid.sand_amt());
    }

    grid.sand_amt().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        );
        assert_eq!(result, "93");
    }
}
