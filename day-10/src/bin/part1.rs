use nom::{
    branch::alt,
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

#[derive(Debug, Clone)]
enum Command {
    Addx(i32),
    Noop,
}

fn addx(input: &str) -> IResult<&str, Command> {
    let (input, (_, value)) = separated_pair(tag("addx"), tag(" "), complete::i32)(input)?;
    Ok((input, Command::Addx(value)))
}

fn noop(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Command::Noop))
}

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(newline, alt((addx, noop)))(input)
}

fn process(input: &str) -> String {
    let (_, commands) = parse(input).expect("should parse");
    dbg!(&commands);

    let mut x = 1;
    let mut current_tick = 0;
    let mut signal_strengths = vec![];

    for command in commands {
        let current_command = command.clone();
        let mut current_command_ticks = match command {
            Command::Addx(_) => 2,
            Command::Noop => 1,
        };

        while current_command_ticks > 0 {
            current_tick += 1;
            current_command_ticks -= 1;

            dbg!(&current_tick, &x);

            if matches!(current_tick, 20 | 60 | 100 | 140 | 180 | 220) {
                signal_strengths.push(x * current_tick);
            }

            match current_command {
                Command::Addx(value) => {
                    if current_command_ticks == 0 {
                        x += value;
                    }
                }
                Command::Noop => {}
            }
        }
    }

    dbg!(&signal_strengths);

    signal_strengths.iter().sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        );
        assert_eq!(result, "13140");
    }
}
