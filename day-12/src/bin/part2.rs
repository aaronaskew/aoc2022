use glam::IVec2;
use pathfinding::prelude::dijkstra;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Puzzle {
    grid: Vec<Vec<u8>>,
    start: IVec2,
    goal: IVec2,
}

fn grid_val(grid: &[Vec<u8>], pos: IVec2) -> u8 {
    grid[pos.y as usize][pos.x as usize]
}

fn valid_moves(position: &IVec2, grid: &[Vec<u8>]) -> Vec<(IVec2, usize)> {
    let mut moves = vec![];
    let curr_height = grid[position.y as usize][position.x as usize];

    //up
    let new_pos = *position + IVec2::NEG_Y;
    if position.y > 0 && curr_height as i32 - grid_val(grid, new_pos) as i32 >= -1 {
        moves.push((*position + IVec2::NEG_Y, 1));
    }

    //down
    let new_pos = *position + IVec2::Y;
    if position.y < grid.len() as i32 - 1
        && curr_height as i32 - grid_val(grid, new_pos) as i32 >= -1
    {
        moves.push((*position + IVec2::Y, 1));
    }

    //left
    let new_pos = *position + IVec2::NEG_X;
    if position.x > 0 && curr_height as i32 - grid_val(grid, new_pos) as i32 >= -1 {
        moves.push((*position + IVec2::NEG_X, 1));
    }

    //right
    let new_pos = *position + IVec2::X;
    if position.x < grid[0].len() as i32 - 1
        && curr_height as i32 - grid_val(grid, new_pos) as i32 >= -1
    {
        moves.push((*position + IVec2::X, 1));
    }

    // dbg!(&position, &moves);

    moves
}

fn process(input: &str) -> String {
    // let mut start_positions = Vec::new();
    let mut goal = IVec2::NEG_X;

    let mut grid: Vec<Vec<u8>> = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, ch) in line.chars().enumerate() {
            row.push(match ch {
                'S' => {
                    // start = IVec2::new(x as i32, y as i32);
                    b'a'
                }
                'E' => {
                    goal = IVec2::new(x as i32, y as i32);
                    b'z'
                }
                _ => ch as u8,
            });
        }
        grid.push(row);
    }

    let start_positions: Vec<IVec2> = grid
        .iter()
        .enumerate()
        .flat_map(move |(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, height)| **height == b'a')
                .map(move |(x, _)| IVec2::new(x as i32, y as i32))
        })
        .collect();

    dbg!(&start_positions);

    let mut results = vec![];

    // dbg!(&grid, &start, &goal);

    for start in start_positions {
        if let Some(result) = dijkstra(
            &start,
            |&position| valid_moves(&position, &grid),
            |&position| position == goal,
        ) {
            results.push(result.1);
        }
    }

    dbg!(&results);

    results.iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        );
        assert_eq!(result, "29");
    }
}
