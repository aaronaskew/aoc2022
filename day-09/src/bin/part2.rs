use std::collections::HashSet;

use glam::IVec2;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn is_adjacent(head: IVec2, tail: IVec2) -> bool {
    head.distance_squared(tail) <= 2
}

fn process(input: &str) -> String {
    let mut visited: HashSet<IVec2> = HashSet::new();

    let mut rope = vec![IVec2::ZERO; 10];


    visited.insert(IVec2::ZERO);

    for line in input.lines() {
        let mut parts = line.split(" ");
        let direction = parts.next().expect("should have a direction");
        let distance = parts
            .next()
            .expect("should have a distance")
            .parse::<u32>()
            .expect("should parse distance");

        // dbg!(&direction, &distance);

        for _ in 0..distance {
            



            rope[0] += match direction {
                "U" => IVec2::NEG_Y,
                "D" => IVec2::Y,
                "L" => IVec2::NEG_X,
                "R" => IVec2::X,
                _ => panic!(),
            };

            for i in 1..10usize {
                if !is_adjacent(rope[i-1], rope[i]) {
                    // move tail
                    let delta = rope[i-1] - rope[i];
                    // dbg!(&delta);
    
                    rope[i] += match (delta.x, delta.y) {
                        (0, 2) => IVec2::Y,
                        (0, -2) => IVec2::NEG_Y,
                        (2, 0) => IVec2::X,
                        (-2, 0) => IVec2::NEG_X,
                        _ => {
                            let x = if delta.x > 0 { 1 } else { -1 };
                            let y = if delta.y > 0 { 1 } else { -1 };
                            IVec2::new(x, y)
                        }
                    };
                }
            }

            

            // dbg!(head, tail);

            visited.insert(rope[9]);
        }
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        assert_eq!(result, "1");
    }

    #[test]
    fn example2() {
        let result = process(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );
        assert_eq!(result, "36");
    }
}
