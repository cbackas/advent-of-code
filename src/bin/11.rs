use itertools::Itertools;
use rayon::{
    prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

advent_of_code::solution!(11);

fn convert_to_bool_vec(input: &str) -> Vec<Vec<bool>> {
    let rows: Vec<Vec<bool>> = input
        .par_lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("unexpected char"),
                })
                .collect()
        })
        .collect();

    rows
}

fn find_empty_rows_and_cols(rows: &Vec<Vec<bool>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    let col_length = rows[0].len();

    let mut col_index = 0;
    let mut row_index = 0;
    loop {
        let current_row = &rows[row_index];

        if col_index == 0 && !current_row.contains(&true) {
            empty_rows.push(row_index);
        }

        if row_index == 0 && !rows.iter().any(|row| row[col_index]) {
            empty_cols.push(col_index);
        }

        col_index += 1;

        if col_index == col_length {
            col_index = 0;
            row_index += 1;
        }

        if row_index == rows.len() {
            break;
        }
    }

    (empty_rows, empty_cols)
}

#[derive(Debug)]
struct Galaxy {
    position: (usize, usize),
}

fn find_galaxies(rows: &Vec<Vec<bool>>, expansion: usize) -> Vec<Galaxy> {
    let (empty_rows, empty_cols) = find_empty_rows_and_cols(&rows);

    let galaxies: Vec<Galaxy> = rows
        .par_iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            let empty_rows = empty_rows.clone();
            let empty_cols = empty_cols.clone();

            row.par_iter()
                .enumerate()
                .filter_map(move |(col_index, &is_galaxy)| {
                    if !is_galaxy {
                        return None;
                    }

                    let applicable_empty_rows = empty_rows
                        .iter()
                        .filter(|&&empty_row| empty_row < row_index)
                        .count();
                    let applicable_empty_cols = empty_cols
                        .iter()
                        .filter(|&&empty_col| empty_col < col_index)
                        .count();

                    let expansion = expansion as usize - 1;
                    let row_index = row_index + applicable_empty_rows * expansion;
                    let col_index = col_index + applicable_empty_cols * expansion;

                    Some(Galaxy {
                        position: (row_index, col_index),
                    })
                })
        })
        .collect();

    galaxies
}

fn pair_galaxies(galaxies: &Vec<Galaxy>) -> Vec<(&Galaxy, &Galaxy)> {
    galaxies.iter().tuple_combinations().collect_vec()
}

fn calculate_distance(galaxy_one: &Galaxy, galaxy_two: &Galaxy) -> u64 {
    let x_distance = (galaxy_two.position.0 as i32 - galaxy_one.position.0 as i32).abs();
    let y_distance = (galaxy_two.position.1 as i32 - galaxy_one.position.1 as i32).abs();

    let distance = x_distance + y_distance;

    distance as u64
}

fn calc_result(input: &str, expansion: usize) -> Option<u64> {
    let rows = convert_to_bool_vec(input);
    let galaxies = find_galaxies(&rows, expansion);
    let galaxies = pair_galaxies(&galaxies);
    let result: u64 = galaxies
        .par_iter()
        .map(|galaxy| {
            let distance = calculate_distance(galaxy.0, galaxy.1);
            distance
        })
        .sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    calc_result(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    calc_result(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
