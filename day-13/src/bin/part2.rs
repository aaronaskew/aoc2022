use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Pair {
    left: Data,
    right: Data,
}

#[derive(Debug, Clone, Eq)]
enum Data {
    Int(i32),
    List(Vec<Data>),
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::List(l), Self::Int(r_int)) => l == &vec![Self::Int(*r_int)],
            (Self::Int(l_int), Self::List(r)) => &vec![Self::Int(*l_int)] == r,
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Data::Int(l), Data::Int(r)) => l.cmp(r),
            (Data::Int(_), Data::List(_)) => Data::List(vec![self.clone()]).cmp(other),
            (Data::List(_), Data::Int(_)) => self.cmp(&Data::List(vec![other.clone()])),
            (Data::List(l), Data::List(r)) => l.cmp(r),
        }
    }
}

fn int(input: &str) -> IResult<&str, Data> {
    let (input, num) = complete::i32(input)?;
    Ok((input, Data::Int(num)))
}

fn list(input: &str) -> IResult<&str, Data> {
    let (input, list_data) = preceded(
        tag("["),
        terminated(separated_list0(tag(","), data), tag("]")),
    )(input)?;

    Ok((input, Data::List(list_data)))
}

fn data(input: &str) -> IResult<&str, Data> {
    alt((list, int))(input)
}

fn pair(input: &str) -> IResult<&str, Pair> {
    let (input, (left, right)) = separated_pair(data, newline, data)(input)?;

    dbg!(&left, &right);

    Ok((input, Pair { left, right }))
}

fn parse(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(tag("\n\n"), pair)(input)
}

fn process(input: &str) -> String {
    let (_input, mut pairs) = parse(input).expect("should parse");

    // dbg!(&_input, &pairs);

    pairs.push(Pair {
        left: Data::List(vec![Data::List(vec![Data::Int(2)])]),
        right: Data::List(vec![Data::List(vec![Data::Int(6)])]),
    });

    let mut data: Vec<Data> = pairs
        .into_iter()
        .flat_map(|p| vec![p.left.clone(), p.right.clone()])
        .collect();

    data.sort();

    dbg!(&data);

    data.iter()
        .enumerate()
        .filter(|(_, d)| {
            **d == Data::List(vec![Data::List(vec![Data::Int(2)])])
                || **d == Data::List(vec![Data::List(vec![Data::Int(6)])])
        })
        .map(|(i, _)| i + 1)
        .inspect(|i| {
            dbg!(&i);
        })
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        );
        assert_eq!(result, "140");
    }
}
