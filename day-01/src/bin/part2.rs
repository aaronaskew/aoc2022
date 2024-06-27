fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut elves = input.split("\n\n")
    .map(|elf_supplies| {
        elf_supplies.lines().map(|line| line.parse::<u64>().unwrap()).sum::<u64>()
    }).collect::<Vec<u64>>();

    elves.sort_by(|a,b| b.cmp(a));

    dbg!(&elves);

    elves.iter().take(3).sum::<u64>().to_string()
 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part2(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );

        assert_eq!(result, "45000");
    }
}
