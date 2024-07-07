use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Sensor {
    position: IVec2,
    closest: IVec2,
}

impl Sensor {
    fn radius(&self) -> i32 {
        self.position.x.abs_diff(self.closest.x) as i32
            + self.position.y.abs_diff(self.closest.y) as i32
    }
}

fn sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, sx) = preceded(tag("Sensor at x="), complete::i32)(input)?;
    let (input, sy) = preceded(tag(", y="), complete::i32)(input)?;
    let (input, bx) = preceded(tag(": closest beacon is at x="), complete::i32)(input)?;
    let (input, by) = preceded(tag(", y="), complete::i32)(input)?;
    Ok((
        input,
        Sensor {
            position: IVec2::new(sx, sy),
            closest: IVec2::new(bx, by),
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(newline, sensor)(input)
}

fn process(input: &str) -> String {
    let (input, sensors) = parse(input).expect("should parse");

    let row = 2_000_000;

    let beacons_on_row = sensors
        .clone()
        .into_iter()
        .filter(|s| s.closest.y == row)
        .map(|s| s.closest)
        .unique()
        .inspect(|beacon| {
            dbg!(beacon);
        })
        .count();

    let visible = sensors
        .iter()
        .filter(|s| {
            let dist = s.position.y.abs_diff(row) as i32;
            let rad = s.radius();

            rad > dist
        })
        .inspect(|s| {
            let dist = s.position.y.abs_diff(row) as i32;
            let rad = s.radius();
            let remainder = rad - dist;

            dbg!(s, dist, rad, remainder);
        })
        .map(|s| {
            let dist = s.position.y.abs_diff(row) as i32;
            let rad = s.radius();
            let remainder = rad - dist;

            let x_range = (s.position.x - remainder)..=(s.position.x + remainder);

            x_range.map(|x| IVec2::new(x,row)).collect::<Vec<IVec2>>()
        }).flatten()
        .unique()
        // .inspect(|x_range| {
        //     dbg!(x_range);
        // })
        .count();

    dbg!(&beacons_on_row, &visible);

    (visible - beacons_on_row).to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        );
        assert_eq!(result, "26");
    }
}
