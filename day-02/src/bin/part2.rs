fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
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

fn part2(input: &str) -> String {
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

            let us = match (opponent, scores[1]) {
                (Some(Rock), "X") => Some(Scissors),
                (Some(Rock), "Y") => Some(Rock),
                (Some(Rock), "Z") => Some(Paper),
                (Some(Paper), "X") => Some(Rock),
                (Some(Paper), "Y") => Some(Paper),
                (Some(Paper), "Z") => Some(Scissors),
                (Some(Scissors), "X") => Some(Paper),
                (Some(Scissors), "Y") => Some(Scissors),
                (Some(Scissors), "Z") => Some(Rock),
                _ => None,
            };

            Throw { opponent, us }
        })
        .collect();

    dbg!(&throws);

    let scores = throws
        .iter()
        .map(|throw| throw.score())
        .collect::<Vec<u64>>();

    dbg!(&scores);

    scores.iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "A Y
B X
C Z",
        );

        assert_eq!(result, "12");
    }
}
