fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ThrowScore {
    Rock,
    Paper,
    Scissors,
}

use ThrowScore::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Throw {
    opponent: Option<ThrowScore>,
    us: Option<ThrowScore>,
}

impl Throw {
    pub fn score(&self) -> u64 {
        let match_score = match (self.opponent.unwrap(), self.us.unwrap()) {
            (Rock, Paper) | (Scissors, Rock) | (Paper, Scissors) => 6_u64,
            (opponent, us) if opponent == us => 3,
            _ => 0,
        };

        let throw_score = match self.us.unwrap() {
            Rock => 1_u64,
            Paper => 2,
            Scissors => 3,
        };

        match_score + throw_score
    }
}

fn part1(input: &str) -> String {
    let throws: Vec<Throw> = input
        .lines()
        .map(|line| {
            let scores = line.split_whitespace().collect::<Vec<&str>>();

            let opponent = match scores[0] {
                "A" => Some(Rock),
                "B" => Some(Paper),
                "C" => Some(Scissors),
                _ => None,
            };

            let us = match scores[1] {
                "X" => Some(Rock),
                "Y" => Some(Paper),
                "Z" => Some(Scissors),
                _ => None,
            };

            Throw { opponent, us }
        })
        .collect();

    dbg!(&throws);

    let scores = throws.iter().map(|throw| throw.score()).collect::<Vec<u64>>();

    dbg!(&scores);

    scores.iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part1(
            "A Y
B X
C Z",
        );

        assert_eq!(result, "15");
    }
}
