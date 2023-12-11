use itertools::Itertools;
use rayon::{prelude::ParallelIterator, str::ParallelString};

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Row {
    start_index: Option<usize>,
    tiles: Vec<Tile>,
}

#[derive(Debug)]
struct Tile {
    is_start: bool,
    directions: Vec<Direction>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<Row> = input
        .par_lines()
        .map(|line| {
            let mut starting_position: Option<usize> = None;
            let tiles: Vec<Tile> = line
                .chars()
                .enumerate()
                .map(|(char_index, c)| match c {
                    '|' => Tile {
                        is_start: false,
                        directions: vec![Direction::North, Direction::South],
                    },
                    '-' => Tile {
                        is_start: false,
                        directions: vec![Direction::East, Direction::West],
                    },
                    'L' => Tile {
                        is_start: false,
                        directions: vec![Direction::North, Direction::East],
                    },
                    'J' => Tile {
                        is_start: false,
                        directions: vec![Direction::North, Direction::West],
                    },
                    '7' => Tile {
                        is_start: false,
                        directions: vec![Direction::South, Direction::West],
                    },
                    'F' => Tile {
                        is_start: false,
                        directions: vec![Direction::South, Direction::East],
                    },
                    'S' => {
                        starting_position = Some(char_index);
                        Tile {
                            is_start: true,
                            directions: vec![
                                Direction::North,
                                Direction::South,
                                Direction::East,
                                Direction::West,
                            ],
                        }
                    }
                    _ => Tile {
                        is_start: false,
                        directions: vec![],
                    },
                })
                .collect();

            Row {
                tiles,
                start_index: starting_position,
            }
        })
        .collect();

    let starting_row = rows
        .iter()
        .enumerate()
        .find(|(_, row)| row.start_index.is_some())
        .unwrap();
    let starting_position = (starting_row.0, starting_row.1.start_index.unwrap());

    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut current_position = starting_position;
    let mut distance = 0;
    let mut last_direction: Option<&Direction> = None;
    loop {
        let current_tile = &rows[current_position.0].tiles[current_position.1];

        if current_tile.is_start && distance > 0 {
            break;
        }

        let directions = &current_tile.directions;
        let viable_directions = directions
            .iter()
            .filter(|direction| match direction {
                Direction::North => {
                    if current_position.0 == 0 {
                        return false;
                    }
                    let next_position = (current_position.0 - 1, current_position.1);
                    let next_tile = &rows[next_position.0].tiles[next_position.1];
                    next_tile.directions.contains(&Direction::South)
                }
                Direction::South => {
                    if current_position.0 == rows.len() - 1 {
                        return false;
                    }
                    let next_position = (current_position.0 + 1, current_position.1);
                    let next_tile = &rows[next_position.0].tiles[next_position.1];
                    next_tile.directions.contains(&Direction::North)
                }
                Direction::East => {
                    if current_position.1 == rows[current_position.0].tiles.len() - 1 {
                        return false;
                    }
                    let next_position = (current_position.0, current_position.1 + 1);
                    let next_tile = &rows[next_position.0].tiles[next_position.1];
                    next_tile.directions.contains(&Direction::West)
                }
                Direction::West => {
                    if current_position.1 == 0 {
                        return false;
                    }
                    let next_position = (current_position.0, current_position.1 - 1);
                    let next_tile = &rows[next_position.0].tiles[next_position.1];
                    next_tile.directions.contains(&Direction::East)
                }
            })
            .filter(|direction| match last_direction {
                Some(last_direction) => match last_direction {
                    Direction::North => *direction != &Direction::South,
                    Direction::South => *direction != &Direction::North,
                    Direction::East => *direction != &Direction::West,
                    Direction::West => *direction != &Direction::East,
                },
                None => true,
            })
            .collect_vec();

        let dir_len = viable_directions.len();
        if dir_len == 0 {
            panic!("no viable directions");
        }

        if viable_directions.len() > 2 {
            panic!("more than two viable direction: {:?}", viable_directions);
        }

        let next_direction = viable_directions[0];

        current_position = match next_direction {
            Direction::North => (current_position.0 - 1, current_position.1),
            Direction::South => (current_position.0 + 1, current_position.1),
            Direction::East => (current_position.0, current_position.1 + 1),
            Direction::West => (current_position.0, current_position.1 - 1),
        };

        path.push(current_position);
        distance += 1;
        last_direction = Some(next_direction);
    }

    Some(distance / 2)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
