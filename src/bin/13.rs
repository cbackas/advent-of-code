use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug, Clone)]
enum ReflectionDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<Vec<Tile>>,
    reflection_direction: Option<ReflectionDirection>,
    reflection_points: Option<(usize, usize)>,
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut rows = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            patterns.push(identify_reflection_direction(&Pattern {
                rows: rows.clone(),
                reflection_direction: None,
                reflection_points: None,
            }));
            rows.clear();
        } else {
            rows.push(
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Rock,
                        '.' => Tile::Ash,
                        _ => panic!("Unknown tile type: {}", c),
                    })
                    .collect(),
            );
        }
    }

    if !rows.is_empty() {
        patterns.push(identify_reflection_direction(&Pattern {
            rows,
            reflection_direction: None,
            reflection_points: None,
        }));
    }

    patterns
}

fn rows_to_cols(rows: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    (0..rows[0].len())
        .into_par_iter()
        .map(|x| rows.par_iter().map(|row| row[x].clone()).collect())
        .collect()
}

fn verify_reflection(tiles: Vec<Vec<Tile>>, starting_index: usize) -> bool {
    let mut offset = 1;
    let mut is_reflection = true;
    loop {
        let offset_forward = tiles.get(starting_index + 1 + offset);
        let offset_backward = tiles.get(starting_index - offset);

        if offset_forward.is_none() && offset_backward.is_none() {
            break;
        }

        if offset_forward.is_none() == !offset_backward.is_none() {
            break;
        }

        let offset_forward = offset_forward.unwrap();
        let offset_backward = offset_backward.unwrap();

        if offset_forward != offset_backward {
            is_reflection = false;
            break;
        }

        offset += 1;
    }

    is_reflection
}

fn identify_reflection_direction(pattern: &Pattern) -> Pattern {
    let mut x = 0;
    let mut y = 0;

    let rows = &pattern.rows;
    let cols = &rows_to_cols(rows);

    loop {
        let current_row = &rows[y];
        let next_row = &rows.get(y + 1);

        if let Some(next_row) = next_row {
            if current_row != next_row.to_owned() {
                y += 1;
                continue;
            } else {
                let is_reflection = verify_reflection(rows.clone(), y);
                if !is_reflection {
                    y += 1;
                    continue;
                }

                return Pattern {
                    rows: rows.clone(),
                    reflection_direction: Some(ReflectionDirection::Horizontal),
                    reflection_points: Some((y + 1, y + 2)),
                };
            }
        }

        let curr_col = &cols[x];
        let next_col = &cols.get(x + 1);
        if let Some(next_col) = next_col {
            if curr_col != next_col.to_owned() {
                x += 1;
            } else {
                let is_reflection = verify_reflection(cols.clone(), x);
                if !is_reflection {
                    x += 1;
                    continue;
                }

                return Pattern {
                    rows: rows.clone(),
                    reflection_direction: Some(ReflectionDirection::Vertical),
                    reflection_points: Some((x + 1, x + 2)),
                };
            }
        }
    }
}

fn calculate_result(pattern: Pattern) -> u32 {
    let index_1 = pattern.reflection_points.unwrap().0;

    match pattern.reflection_direction.unwrap() {
        ReflectionDirection::Horizontal => (index_1 as u32) * 100,
        ReflectionDirection::Vertical => index_1 as u32,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let patterns = parse_patterns(input);
    let result = patterns
        .into_par_iter()
        .map(|pattern| calculate_result(pattern))
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
        assert_eq!(result, Some(405));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
