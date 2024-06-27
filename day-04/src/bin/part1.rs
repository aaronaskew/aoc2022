use std::ops::RangeInclusive;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (_, ranges) = parse_ranges(input).unwrap();

    dbg!(&ranges);

    ranges
        .iter()
        .filter(|range| {
            let range_0 = range.0.clone();
            let range_1 = range.1.clone();

            dbg!(&range_0, &range_1);

            (range_1.contains(range_0.start()) && range_1.contains(range_0.end()))
                || (range_0.contains(range_1.start()) && range_0.contains(range_1.end()))
        })
        .count()
        .to_string()
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>> {
    let (input, numbers) = many1(terminated(
        separated_list1(alt((tag(","), tag("-"))), nom::character::complete::u64),
        multispace0,
    ))(input)?;

    Ok((
        input,
        numbers
            .iter()
            .map(|range_values| {
                (
                    range_values[0]..=range_values[1],
                    range_values[2]..=range_values[3],
                )
            })
            .collect(),
    ))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );

        assert_eq!(result, "2");
    }
}
