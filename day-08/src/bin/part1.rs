fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn is_visible(grid: &[Vec<u8>], tree_x: usize, tree_y: usize) -> bool {
    let width = grid[0].len();
    let height = grid.len();

    if tree_x == 0 || tree_x == width - 1 || tree_y == 0 || tree_y == height - 1 {
        return true;
    }

    let tree_height = grid[tree_y][tree_x];

    // from above
    if (0..tree_y)
        .map(|y| grid[y][tree_x])
        .all(|height| height < tree_height)
    {
        return true;
    }

    // from below
    if ((tree_y + 1)..height)
        .map(|y| grid[y][tree_x])
        .all(|height| height < tree_height)
    {
        return true;
    }

    // from the left
    if (0..tree_x)
        .map(|x| grid[tree_y][x])
        .all(|height| height < tree_height)
    {
        return true;
    }

    // from the right
    if ((tree_x + 1)..width)
        .map(|x| grid[tree_y][x])
        .all(|height| height < tree_height)
    {
        return true;
    }

    false
}

fn scenic_score(grid: &[Vec<u8>], tree_x: usize, tree_y: usize) -> u32 {
    let width = grid[0].len();
    let height = grid.len();

    if tree_x == 0 || tree_x == width - 1 || tree_y == 0 || tree_y == height - 1 {
        return 0;
    }

    let tree_height = grid[tree_y][tree_x];

    let mut up = 0u32;
    for y in (0..tree_y).rev() {
        up += 1;
        if grid[y][tree_x] >= tree_height {
            break;
        }
    }

    let mut down = 0u32;
    for y in (tree_y + 1)..height {
        down += 1;
        if grid[y][tree_x] >= tree_height {
            break;
        }
    }

    let mut left = 0u32;
    for x in (0..tree_x).rev() {
        left += 1;
        if grid[tree_y][x] >= tree_height {
            break;
        }
    }

    let mut right = 0u32;
    for x in (tree_x + 1)..width {
        right += 1;
        if grid[tree_y][x] >= tree_height {
            break;
        }
    }

    up * down * left * right
}

fn process(input: &str) -> String {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("should parse as u32") as u8)
                .collect()
        })
        .collect();

    let mut scenic_scores = vec![];

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            scenic_scores.push(scenic_score(&grid, x, y))
        }
    }

    scenic_scores.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "30373
25512
65332
33549
35390",
        );
        assert_eq!(result, "21");
    }
}
