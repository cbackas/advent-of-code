use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    sync::RwLock,
};

use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MirrorType {
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SplitterDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TileType {
    Empty,
    Mirror(MirrorType),
    Splitter(SplitterDirection),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    position: (usize, usize),
    tile_type: TileType,
}

impl Tile {
    fn new(position: (usize, usize), tile_type: TileType) -> Self {
        Self {
            position,
            tile_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PathTile {
    tile: Tile,
    direction: Direction,
    is_edge: bool,
}

// fn print_grid(coords: Vec<(usize, usize)>) {
//     println!("{:?}", coords);
//
//     let mut grid = vec![vec!['.'; 10]; 10]; // Initialize a 10x10 grid with '.'
//
//     // Mark the coordinates with '#'
//     for (row, col) in coords {
//         grid[row][col] = '#';
//     }
//
//     // Print the grid
//     for row in grid {
//         for col in row {
//             print!("{}", col);
//         }
//         println!(); // New line at the end of each row
//     }
// }
//
// Custom key for memoization
// #[derive(Eq, Clone, Debug, Hash, PartialEq)]
// struct MemoKey {
//     start: (usize, usize),
//     current_direction: Direction,
// }

static MEMO: Lazy<RwLock<HashMap<Edge, u32>>> = Lazy::new(|| RwLock::new(HashMap::new()));

fn memoized_light_path_find(
    rows: Vec<String>,
    start: (usize, usize),
    current_direction: Direction,
) -> u32 {
    {
        let key = Edge::new(start.0, start.1, current_direction);
        let memo_read = MEMO.read().unwrap();
        if let Some(result) = memo_read.get(&key) {
            // println!("cache hit");
            return *result;
        }
    }

    let path = light_path_find(rows, start, current_direction, HashSet::new());

    let edges: HashSet<Edge> = path
        .par_iter()
        .filter_map(|path_tile| {
            if !path_tile.is_edge {
                return None;
            }
            let position = path_tile.tile.position;
            let direction = match path_tile.direction {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            };
            Some(Edge::new(position.0, position.1, direction))
        })
        .collect();

    let unique_positions: HashSet<(usize, usize)> = path
        .iter()
        .map(|path_tile| path_tile.tile.position)
        .collect();
    let charged = unique_positions.len() as u32;

    {
        let mut memo_write = MEMO.write().unwrap();
        for edge in edges {
            let _result = memo_write.insert(edge, charged);
            // if result.is_some() {
            //     println!("cache collision");
            // }
        }
    }

    charged
}

fn light_path_find(
    rows: Vec<String>,
    start: (usize, usize),
    current_direction: Direction,
    path: HashSet<PathTile>,
) -> HashSet<PathTile> {
    let mut path = path.clone();

    let mut current_direction = current_direction.clone();

    let mut row = start.0;
    let mut col = start.1;

    loop {
        // check if the current position is out of bounds
        if row >= rows.len() || col >= rows[row].len() {
            break;
        }

        let current_type = match rows[row].chars().nth(col) {
            Some('.') => TileType::Empty,
            Some('/') => TileType::Mirror(MirrorType::Forward),
            Some('\\') => TileType::Mirror(MirrorType::Backward),
            Some('-') => TileType::Splitter(SplitterDirection::Horizontal),
            Some('|') => TileType::Splitter(SplitterDirection::Vertical),
            Some(_) => panic!("Invalid character"),
            None => panic!("Character not found"),
        };

        let new_directions: Vec<Direction> = match current_type {
            TileType::Empty => vec![current_direction],
            TileType::Mirror(MirrorType::Forward) => match current_direction {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            TileType::Mirror(MirrorType::Backward) => match current_direction {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            TileType::Splitter(SplitterDirection::Horizontal) => match current_direction {
                Direction::Left => vec![Direction::Left],
                Direction::Right => vec![Direction::Right],
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            },
            TileType::Splitter(SplitterDirection::Vertical) => match current_direction {
                Direction::Up => vec![Direction::Up],
                Direction::Down => vec![Direction::Down],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            },
        };

        let mut done = false;
        for new_direction in &new_directions {
            // check if moving in the new direction would be out of bounds
            let moves_out_of_bounds = match new_direction {
                Direction::Up => row == 0,
                Direction::Down => row == rows.len() - 1,
                Direction::Left => col == 0,
                Direction::Right => col == rows[row].len() - 1,
            };

            let path_tile = PathTile {
                tile: Tile::new((row, col), current_type),
                direction: *new_direction,
                is_edge: moves_out_of_bounds,
            };
            let inserted = path.insert(path_tile);
            if !inserted {
                done = true;
                break;
            }
        }
        if done {
            break;
        }

        if current_type == TileType::Empty {
            match current_direction {
                Direction::Up => {
                    if row == 0 {
                        break;
                    }
                    row -= 1
                }
                Direction::Down => {
                    if row == rows.len() - 1 {
                        break;
                    }
                    row += 1
                }
                Direction::Left => {
                    if col == 0 {
                        break;
                    }
                    col -= 1
                }
                Direction::Right => {
                    if col == rows[row].len() - 1 {
                        break;
                    }
                    col += 1
                }
            }
            continue;
        }

        if current_type == TileType::Mirror(MirrorType::Forward) {
            match current_direction {
                Direction::Up => {
                    if col == rows[row].len() - 1 {
                        break;
                    }
                    current_direction = Direction::Right;
                    col += 1;
                }
                Direction::Down => {
                    if col == 0 {
                        break;
                    }
                    current_direction = Direction::Left;
                    col -= 1;
                }
                Direction::Left => {
                    if row == rows.len() - 1 {
                        break;
                    }
                    current_direction = Direction::Down;
                    row += 1;
                }
                Direction::Right => {
                    if row == 0 {
                        break;
                    }
                    current_direction = Direction::Up;
                    row -= 1;
                }
            }
            continue;
        } else if current_type == TileType::Mirror(MirrorType::Backward) {
            match current_direction {
                Direction::Up => {
                    if col == 0 {
                        break;
                    }
                    current_direction = Direction::Left;
                    col -= 1;
                }
                Direction::Down => {
                    if col == rows[row].len() - 1 {
                        break;
                    }
                    current_direction = Direction::Right;
                    col += 1;
                }
                Direction::Left => {
                    if row == 0 {
                        break;
                    }
                    current_direction = Direction::Up;
                    row -= 1;
                }
                Direction::Right => {
                    if row == rows.len() - 1 {
                        break;
                    }
                    current_direction = Direction::Down;
                    row += 1;
                }
            }
            continue;
        }

        if current_type == TileType::Splitter(SplitterDirection::Horizontal) {
            match current_direction {
                Direction::Left => col -= 1,
                Direction::Right => col += 1,
                Direction::Up | Direction::Down => {
                    if col != 0 {
                        let left_path = light_path_find(
                            rows.clone(),
                            (row, col - 1),
                            Direction::Left,
                            path.clone(),
                        );
                        path.extend(left_path);
                    }

                    if col != rows[row].len() - 1 {
                        let right_path =
                            light_path_find(rows, (row, col + 1), Direction::Right, path.clone());
                        path.extend(right_path);
                    }

                    break;
                }
            }
        } else if current_type == TileType::Splitter(SplitterDirection::Vertical) {
            match current_direction {
                Direction::Up => row -= 1,
                Direction::Down => row += 1,
                Direction::Left | Direction::Right => {
                    if row != 0 {
                        let up_path = light_path_find(
                            rows.clone(),
                            (row - 1, col),
                            Direction::Up,
                            path.clone(),
                        );
                        path.extend(up_path);
                    }

                    if row < rows.len() - 1 {
                        let down_path = light_path_find(
                            rows.clone(),
                            (row + 1, col),
                            Direction::Down,
                            path.clone(),
                        );
                        path.extend(down_path);
                    }

                    break;
                }
            }
        }
    }

    path
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Edge {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Edge {
    fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {
            row,
            col,
            direction,
        }
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
        // Map opposite directions to the same value for hashing
        let direction_value = match self.direction {
            Direction::Up | Direction::Down => 0,
            Direction::Left | Direction::Right => 1,
        };
        direction_value.hash(state);
    }
}

fn edge_indexes<T>(vec_2d: &Vec<Vec<T>>) -> Vec<Edge> {
    let mut edges = Vec::new();

    if vec_2d.is_empty() {
        return edges;
    }

    let row_count = vec_2d.len();
    let col_count = vec_2d[0].len();

    // top and bottom rows
    for col in 0..col_count {
        // top row
        edges.push(Edge::new(0, col, Direction::Down));
        if row_count > 1 {
            // bottom row
            edges.push(Edge::new(row_count - 1, col, Direction::Up));
        }
    }

    // Left and right columns (excluding corners already added)
    for row in 1..row_count - 1 {
        // left collumn
        edges.push(Edge::new(row, 0, Direction::Right));
        if col_count > 1 {
            // right column
            edges.push(Edge::new(row, col_count - 1, Direction::Left));
        }
    }

    edges
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows = input.lines().map(|r| r.to_string()).collect_vec();

    let light_path = light_path_find(rows, (0, 0), Direction::Right, HashSet::new());
    let unique_positions: HashSet<(usize, usize)> = light_path
        .iter()
        .map(|path_tile| path_tile.tile.position)
        .collect();

    Some(unique_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = input.lines().map(|r| r.to_string()).collect_vec();

    let edges = edge_indexes(&rows.iter().map(|r| r.chars().collect_vec()).collect_vec());
    let result = edges
        .par_iter()
        .map(|edge| {
            let light_path = memoized_light_path_find(
                rows.clone(),
                (edge.row, edge.col),
                edge.direction.clone(),
            );
            light_path
        })
        .max();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
