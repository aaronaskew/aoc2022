fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut elves = input.split("\n\n")
    .map(|elf_supplies| {
        elf_supplies.lines().map(|line| line.parse::<u64>().unwrap()).sum::<u64>()
    }).collect::<Vec<u64>>();

    elves.sort();

    dbg!(&elves);

    elves.last().unwrap().to_string()
    
 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = part1(
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

        assert_eq!(result, "24000");
    }
}
