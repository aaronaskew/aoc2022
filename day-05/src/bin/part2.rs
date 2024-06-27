use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{anychar, multispace0, u64},
    multi::{many1, many_till},
    IResult,
};
use nom_locate::{position, LocatedSpan};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let input = Span::new(input);

    println!("{input}");

    let (input, mut stacks) = parse_crates(input).expect("to parse crates");
    let (_, instructions) = parse_instructions(input).expect("to parse instructions");

    // instructions.reverse();

    dbg!(&stacks, &instructions);

    let mut result = String::new();
    for instruction in instructions {
        dbg!(instruction);

        // let  source_stack = stacks.get_mut(instruction.source_stack as usize).unwrap();
        // let  destination_stack = stacks.get_mut(instruction.destination_stack as usize).unwrap();

        let mut crates_in_transit: Vec<Crate> = Vec::new();

        for _ in 0..instruction.quantity {
            crates_in_transit.push(
                stacks
                    .get_mut(instruction.source_stack as usize - 1)
                    .unwrap()
                    .crates
                    .pop()
                    .unwrap(),
            );
        }

        // dbg!(&stacks, &crates_in_transit);

        for c in crates_in_transit.iter().rev() {
            stacks
                .get_mut(instruction.destination_stack as usize - 1)
                .unwrap()
                .crates
                .push(*c);
        }

        dbg!(&stacks);
    }

    dbg!(&stacks);

    for stack in &stacks {
        result.push(dbg!(stack.crates.last().unwrap().label));
    }
    result
}

#[derive(Clone, Copy)]
struct Crate<'a> {
    label: char,
    init_position: Span<'a>,
}

impl std::fmt::Debug for Crate<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[derive(Debug)]
struct Stack<'a> {
    crates: Vec<Crate<'a>>,
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse_crates(s: Span) -> IResult<Span, Vec<Stack>> {
    let mut stacks: Vec<Stack> = Vec::new();

    let (s, res) = many_till(parse_crate, nom::character::complete::digit1)(s)?;
    let (s, _) = take_until("move")(s)?;

    //dbg!(&res.0);

    // for c in &res.0 {
    //     dbg!(
    //         c.init_position.location_line(),
    //         c.init_position.get_column() / 4 + 1
    //     );
    // }

    let mut crates = res.0;

    crates.reverse();

    //dbg!(&crates);

    for c in &crates {
        let idx = c.init_position.get_column() / 4;

        while idx >= stacks.len() {
            stacks.push(Stack { crates: Vec::new() })
        }

        let stack = stacks.get_mut(idx).unwrap();

        stack.crates.push(*c);
    }

    Ok((s, stacks))

    // dbg!(position.location_line(), (position.get_column() as f32 / 4.0 ).ceil() as usize);
}

fn parse_crate(s: Span) -> IResult<Span, Crate> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("[")(s)?;
    let (s, position) = position(s)?;
    let (s, c) = anychar(s)?;
    let (s, _) = tag("]")(s)?;
    let (s, _) = multispace0(s)?;

    Ok((
        s,
        Crate {
            label: c,
            init_position: position,
        },
    ))
}

fn parse_instructions(s: Span) -> IResult<Span, Vec<Instruction>> {
    let (s, instructions) = many1(parse_instruction)(s)?;

    Ok((s, instructions))
}

fn parse_instruction(s: Span) -> IResult<Span, Instruction> {
    let (s, _) = tag("move ")(s)?;
    let (s, quantity) = u64(s)?;
    let (s, _) = tag(" from ")(s)?;
    let (s, source_stack) = u64(s)?;
    let (s, _) = tag(" to ")(s)?;
    let (s, destination_stack) = u64(s)?;
    let (s, _) = multispace0(s)?;

    Ok((
        s,
        Instruction {
            quantity,
            source_stack,
            destination_stack,
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    quantity: u64,
    source_stack: u64,
    destination_stack: u64,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );

        assert_eq!(result, "MCD");
    }
}
