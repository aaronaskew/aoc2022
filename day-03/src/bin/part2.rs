#![feature(iter_array_chunks)]

use std::collections::BTreeMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let letter_score = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<BTreeMap<char, usize>>();

    input
        .lines()
        .array_chunks::<3>()
        .map(|[inventory_a, inventory_b, inventory_c]| {
            dbg!(inventory_a, inventory_b, inventory_c);

            let badge = *inventory_a
                .chars()
                .filter(|c| inventory_b.contains(*c) && inventory_c.contains(*c))
                .take(1)
                .map(|c| letter_score.get(&c).unwrap()).next().unwrap();

            dbg!(badge);

            badge
        })
        .sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(result, "70");
    }
}
