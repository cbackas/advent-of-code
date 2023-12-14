use rayon::{
    prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
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
    East,
    South,
    West,
}

fn print_map(tiles: &Vec<Vec<Tile>>, prefix: &str) {
    println!("{}:", prefix);
    for row in tiles {
        for tile in row {
            print!(
                "{}",
                match tile {
                    Tile::Solid => '#',
                    Tile::Empty => '.',
                    Tile::Movable => 'O',
                }
            );
        }
        println!();
    }
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

fn tilt_north(tiles: &mut Vec<Vec<Tile>>) {
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
}

fn calculate_weight(tiles: &Vec<Vec<Tile>>, direction: Direction) -> u32 {
    tiles
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.par_iter()
                .enumerate()
                .filter_map(|(y, tile)| {
                    if tile != &Tile::Movable {
                        return None;
                    }

                    let weight = match direction {
                        Direction::North => tiles.len() as u32 - x as u32,
                        _ => unreachable!("Unknown direction: {:?}", direction),
                    };

                    // println!("weight: {}", weight);

                    Some(weight)
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut tiles: Vec<Vec<Tile>> = input.par_lines().map(map_tiles).collect();

    tilt_north(&mut tiles);

    Some(calculate_weight(&tiles, Direction::North))
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
        assert_eq!(result, Some(136));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
