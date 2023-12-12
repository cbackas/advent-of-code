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

fn expand_empty_space(
    rows: Vec<Vec<bool>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
) -> Vec<Vec<bool>> {
    let mut rows = rows;

    let mut empty_cols = empty_cols;
    empty_cols.reverse();
    for empty_col in empty_cols {
        for row_index in 0..rows.len() {
            rows[row_index].insert(empty_col, false);
        }
    }

    let mut empty_rows = empty_rows;
    empty_rows.reverse();
    for empty_row in &empty_rows {
        rows.insert(*empty_row, vec![false; rows[0].len()]);
    }

    rows.to_vec()
}

#[derive(Debug)]
struct Galaxy {
    position: (usize, usize),
}

fn calculate_distance(galaxy_one: &Galaxy, galaxy_two: &Galaxy) -> u32 {
    let x_distance = (galaxy_two.position.0 as i32 - galaxy_one.position.0 as i32).abs();
    let y_distance = (galaxy_two.position.1 as i32 - galaxy_one.position.1 as i32).abs();

    let distance = x_distance + y_distance;

    distance as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let rows = convert_to_bool_vec(input);
    let (empty_rows, empty_cols) = find_empty_rows_and_cols(&rows);
    let rows = expand_empty_space(rows, empty_rows, empty_cols);

    let galaxies: Vec<Galaxy> = rows
        .par_iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.par_iter()
                .enumerate()
                .filter_map(move |(col_index, &is_galaxy)| {
                    if is_galaxy {
                        Some(Galaxy {
                            position: (row_index, col_index),
                        })
                    } else {
                        None
                    }
                })
        })
        .collect();

    let mut galaxy_pairs = Vec::new();
    for (i, first_galaxy) in galaxies.iter().enumerate() {
        for second_galaxy in galaxies.iter().skip(i + 1) {
            galaxy_pairs.push((first_galaxy, second_galaxy));
        }
    }

    let result: u32 = galaxy_pairs
        .par_iter()
        .map(|galaxy| {
            let distance = calculate_distance(galaxy.0, galaxy.1);
            distance
        })
        .sum();

    Some(result)
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
        assert_eq!(result, Some(374));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
