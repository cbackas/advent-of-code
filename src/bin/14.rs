use memoize::memoize;
use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Movable,
    Solid,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
}

fn map_tiles(line: &str) -> Vec<Tile> {
    line.par_chars()
        .map(|c| match c {
            '#' => Tile::Solid,
            '.' => Tile::Empty,
            'O' => Tile::Movable,
            _ => unreachable!("Unknown tile type: {}", c),
        })
        .collect()
}

fn tilt_north(tiles: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut tiles = tiles;
    // this shifts the movable tiles north
    let mut current_position = (0, 0);
    let mut lap_changed_tiles = false;
    loop {
        if current_position.1 > tiles.len() - 1 && current_position.0 > tiles[0].len() - 1 {
            break;
        }

        let current_tile = &tiles[current_position.0][current_position.1];
        let has_row_above = current_position.0 > 0;

        if current_tile == &Tile::Movable {
            if has_row_above {
                let tile_above = &tiles[current_position.0 - 1][current_position.1];
                if tile_above == &Tile::Empty {
                    tiles[current_position.0 - 1][current_position.1] = Tile::Movable;
                    tiles[current_position.0][current_position.1] = Tile::Empty;
                    lap_changed_tiles = true;
                }
            }
        }

        current_position.0 = if current_position.1 == tiles[0].len() {
            0
        } else if current_position.0 == tiles.len() - 1 {
            0
        } else {
            current_position.0 + 1
        };

        if current_position.0 == 0 {
            if current_position.1 == tiles.len() - 1 {
                // restart the whole loop
                current_position.1 = 0;

                // print_map(&tiles, "lap");

                if !lap_changed_tiles {
                    break;
                } else {
                    lap_changed_tiles = false;
                }
            } else {
                current_position.1 += 1;
            }
        }
    }

    tiles
}

fn rotate(tiles: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut tiles = tiles;

    let size = tiles.len();
    let layer_count = size / 2;

    for layer in 0..layer_count {
        let first = layer;
        let last = size - first - 1;

        for element in first..last {
            let offset = element - first;

            let top = tiles[first][element].clone();
            let right_side = tiles[element][last].clone();
            let bottom = tiles[last][last - offset].clone();
            let left_side = tiles[last - offset][first].clone();

            tiles[first][element] = left_side;
            tiles[element][last] = top;
            tiles[last][last - offset] = right_side;
            tiles[last - offset][first] = bottom;
        }
    }

    tiles
}

fn calculate_weight(tiles: &Vec<Vec<Tile>>, direction: Direction) -> u32 {
    tiles
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.par_iter()
                .filter_map(|tile| {
                    if tile != &Tile::Movable {
                        return None;
                    }

                    let weight = match direction {
                        Direction::North => tiles.len() as u32 - x as u32,
                    };

                    // println!("weight: {}", weight);

                    Some(weight)
                })
                .sum::<u32>()
        })
        .sum()
}

#[memoize]
fn spin_cycle(tiles: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut cycle_tyles = tiles.clone();
    for _ in 0..4 {
        cycle_tyles = tilt_north(cycle_tyles);
        cycle_tyles = rotate(cycle_tyles);
    }
    cycle_tyles
}

pub fn part_one(input: &str) -> Option<u32> {
    let tiles: Vec<Vec<Tile>> = input.par_lines().map(map_tiles).collect();
    let tiles = tilt_north(tiles);

    Some(calculate_weight(&tiles, Direction::North))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tiles: Vec<Vec<Tile>> = input.par_lines().map(map_tiles).collect();

    for _i in 0..1000 {
        tiles = spin_cycle(tiles);
    }

    Some(calculate_weight(&tiles, Direction::North))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
