use std::collections::BTreeMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let letter_score = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<BTreeMap<char, usize>>();

    dbg!(&letter_score);

    input
        .lines()
        .map(|line| {
            let (compartment_a, compartment_b) = line.split_at(line.len() / 2);

            let result = compartment_a
                .chars()
                .find(|c| compartment_b.contains(*c))
                .unwrap();

            letter_score.get(&result).unwrap()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part1(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(result, "157");
    }
}
