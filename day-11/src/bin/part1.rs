use core::panic;
use std::collections::VecDeque;

use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::{self, anychar, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone)]
enum Operand {
    Number(u32),
    Old,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(Operand),
    Multiply(Operand),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    test_divisor: u32,
    test_true_monkey: usize,
    test_false_monkey: usize,
    inspect_count: usize,
}

impl Monkey {}

struct Barrel {
    monkeys: Vec<Monkey>,
}

impl Barrel {
    fn do_round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(mut item) = self.monkeys[i].items.pop_front() {
                item = match self.monkeys[i].operation.clone() {
                    Operation::Add(value) => match value {
                        Operand::Number(value) => item + value,
                        Operand::Old => item + item,
                    },
                    Operation::Multiply(value) => match value {
                        Operand::Number(value) => item * value,
                        Operand::Old => item * item,
                    },
                };

                self.monkeys[i].inspect_count += 1;

                item /= 3;

                let test_true_monkey = self.monkeys[i].test_true_monkey;
                let test_false_monkey = self.monkeys[i].test_false_monkey;

                if item % self.monkeys[i].test_divisor == 0 {
                    self.monkeys[test_true_monkey].items.push_back(item);
                } else {
                    self.monkeys[test_false_monkey].items.push_back(item);
                }
            }
        }
    }

    fn monkey_business(&self) -> usize {
        let mut inspect_counts: Vec<usize> = self.monkeys.iter().map(|m| m.inspect_count).collect();
        inspect_counts.sort();
        inspect_counts.reverse();

        dbg!(&inspect_counts);

        inspect_counts[0] * inspect_counts[1]
    }
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, (operator, operand)) = terminated(
        preceded(
            tag("  Operation: new = old "),
            separated_pair(take_until(" "), tag(" "), take_until("\n")),
        ),
        newline,
    )(input)?;

    let operand = if operand == "old" {
        Operand::Old
    } else {
        Operand::Number(operand.parse::<u32>().expect("should parse u32"))
    };

    Ok((
        input,
        match operator {
            "*" => Operation::Multiply(operand),
            "+" => Operation::Add(operand),
            _ => panic!("invalid operator"),
        },
    ))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = terminated(take_until("\n"), newline)(input)?;
    let (input, items) = preceded(
        tag("  Starting items: "),
        terminated(separated_list1(tag(", "), complete::u32), newline),
    )(input)?;
    let (input, operation) = operation(input)?;
    let (input, test_divisor) = terminated(
        preceded(tag("  Test: divisible by "), complete::u32),
        newline,
    )(input)?;
    let (input, test_true_monkey) = terminated(
        preceded(tag("    If true: throw to monkey "), complete::u32).map(|v| v as usize),
        newline,
    )(input)?;
    let (input, test_false_monkey) = preceded(
        tag("    If false: throw to monkey "),
        complete::u32.map(|v| v as usize),
    )(input)?;

    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            operation,
            test_divisor,
            test_true_monkey,
            test_false_monkey,
            inspect_count: 0,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), monkey)(input)
}

fn process(input: &str) -> String {
    let (_, monkeys) = parse(input).expect("should parse");

    let mut barrel = Barrel { monkeys };

    dbg!(&barrel.monkeys);

    for _ in 0..20 {
        barrel.do_round();
    }
    dbg!(&barrel.monkeys);

    barrel.monkey_business().to_string()

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        );
        assert_eq!(result, "10605");
    }
}
