use std::collections::VecDeque;



fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {

    let mut result = 0_usize;

    let mut buffer = Buffer { buffer: VecDeque::new() };


for (i,c) in input.chars().enumerate() {
    buffer.insert(c);
    if buffer.is_unique() {
        result = i + 1;
        break;
    }
}



    result.to_string()
}

#[derive(Debug)]
struct Buffer {
    buffer: VecDeque<char>,
}

impl Buffer {
    fn insert(&mut self, c: char) {
        self.buffer.push_back(c);

        if self.buffer.len() > 4 {
            self.buffer.pop_front();
        }
    }
    
    fn is_unique(&self) -> bool {
        let mut chars: Vec<char> = self.buffer.iter().copied().collect();
        chars.sort();
        chars.dedup();
        chars.len() == 4
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, "7");
        let result = process("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, "5");
        let result = process("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, "6");
        let result = process("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, "10");
        let result = process("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, "11");
    }
}
