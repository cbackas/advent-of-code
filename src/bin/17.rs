use pathfinding::prelude::{dijkstra, Matrix};

advent_of_code::solution!(17);

fn dijkstra_wrapper_wrapper(input: &str, min_moves: u32, max_moves: u32) -> u32 {
    let lava_matrix = Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .unwrap();

    let (_path, heat_loss) = dijkstra(
        &((0, 0), (0, 0), 0),
        |&(pos, (move_row, move_col), moves)| {
            let mut successors = Vec::new();

            let mut move_grid = |dir, l| {
                successors.extend(
                    &lava_matrix
                        .move_in_direction(pos, dir)
                        .map(|pos| ((pos, dir, l), lava_matrix[pos])),
                );
            };

            if moves >= min_moves {
                move_grid((-move_col, -move_row), 1);
                move_grid((move_col, move_row), 1);
            } else if moves == 0 {
                move_grid((1, 0), 1);
                move_grid((0, 1), 1);
            }

            if moves < max_moves {
                move_grid((move_row, move_col), moves + 1);
            }

            successors
        },
        |&(pos, _, _)| pos == (lava_matrix.rows - 1, lava_matrix.columns - 1),
    )
    .unwrap();

    heat_loss
}

pub fn part_one(input: &str) -> Option<u32> {
    let heat_loss = dijkstra_wrapper_wrapper(input, 1, 3);
    Some(heat_loss)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let heat_loss = dijkstra_wrapper_wrapper(_input, 4, 10);
    Some(heat_loss)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71));
    }
}
