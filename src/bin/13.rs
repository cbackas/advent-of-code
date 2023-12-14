use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ReflectionDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Reflection {
    direction: ReflectionDirection,
    points: (usize, usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pattern {
    rows: Vec<Vec<Tile>>,
    reflection: Option<Reflection>,
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut rows = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            let mut pattern = Pattern {
                rows: rows.clone(),
                reflection: None,
            };
            pattern.reflection = identify_reflection(&pattern);
            patterns.push(pattern);
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
        let mut pattern = Pattern {
            rows: rows.clone(),
            reflection: None,
        };
        pattern.reflection = identify_reflection(&pattern);
        patterns.push(pattern);
    }

    patterns
}

fn rows_to_cols(rows: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    (0..rows[0].len())
        .into_par_iter()
        .map(|x| rows.par_iter().map(|row| row[x].clone()).collect())
        .collect()
}

fn verify_reflection(tiles: Vec<Vec<Tile>>, starting_index: usize) -> bool {
    let mut offset = 0;
    let mut is_reflection = true;

    loop {
        let backward_index: isize = (starting_index as isize) - (offset as isize);
        if backward_index < 0 {
            break;
        }
        let forward_index: isize = (starting_index as isize) + (offset as isize);
        if forward_index >= tiles.len() as isize {
            break;
        }

        let offset_backward = tiles.get(starting_index - offset);
        let offset_forward = tiles.get(starting_index + 1 + offset);

        if offset_forward.is_none() && offset_backward.is_none() {
            break;
        }

        if offset_forward.is_none() != offset_backward.is_none() {
            break;
        }

        let offset_backward = offset_backward.unwrap();
        let offset_forward = offset_forward.unwrap();

        if offset_forward != offset_backward {
            is_reflection = false;
            break;
        }

        offset += 1;
    }

    is_reflection
}

fn identify_reflection(pattern: &Pattern) -> Option<Reflection> {
    let mut x = 0;
    let mut y = 0;

    let rows = &pattern.rows;
    let cols = &rows_to_cols(rows.to_vec());

    loop {
        if y >= rows.len() || x >= cols.len() {
            break;
        }

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

                let reflection = Reflection {
                    direction: ReflectionDirection::Horizontal,
                    points: (y + 1, y + 2),
                };

                if let Some(existing_reflection) = &pattern.reflection {
                    if *existing_reflection == reflection {
                        y += 1;
                        continue;
                    }
                }

                return Some(reflection);
            }
        }

        let curr_col = &cols[x];
        let next_col = &cols.get(x + 1);
        if let Some(next_col) = next_col {
            if curr_col != next_col.to_owned() {
                x += 1;
                continue;
            } else {
                let is_reflection = verify_reflection(cols.clone(), x);
                if !is_reflection {
                    x += 1;
                    continue;
                }

                let reflection = Reflection {
                    direction: ReflectionDirection::Vertical,
                    points: (x + 1, x + 2),
                };

                if let Some(existing_reflection) = &pattern.reflection {
                    if *existing_reflection == reflection {
                        x += 1;
                        continue;
                    }
                }

                return Some(reflection);
            }
        }

        break;
    }

    None
}

fn calculate_result(pattern: Pattern) -> u32 {
    let reflection = pattern.reflection.unwrap();
    let index_1 = reflection.points.0;

    match reflection.direction {
        ReflectionDirection::Horizontal => (index_1 as u32) * 100,
        ReflectionDirection::Vertical => index_1 as u32,
    }
}

fn smudge_finder(pattern: Pattern, current_position: (usize, usize)) -> Pattern {
    let x = current_position.0;
    let y = current_position.1;

    let rows = &pattern.rows;
    let col_len = rows[0].len();

    let current_value = rows.get(y).and_then(|row| row.get(x));
    if current_value.is_none() {
        panic!(
            "gone out of bounds checking for smudges: {:?} (row len: {})",
            current_position, col_len
        );
    }

    let current_value = current_value.unwrap();
    let mut rows = rows.clone();
    rows[y][x] = match current_value {
        Tile::Ash => Tile::Rock,
        Tile::Rock => Tile::Ash,
    };
    let mut new_pattern = Pattern {
        rows: rows.clone(),
        reflection: pattern.reflection.clone(),
    };

    if let Some(reflection) = identify_reflection(&new_pattern) {
        new_pattern.reflection = Some(reflection);
        return new_pattern;
    }

    let new_x = if x == col_len - 1 { 0 } else { x + 1 };
    let new_y = if new_x == 0 { y + 1 } else { y };

    smudge_finder(pattern, (new_x, new_y))
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = parse_patterns(input)
        .into_par_iter()
        .map(|pattern| calculate_result(pattern))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = parse_patterns(input)
        .into_par_iter()
        .map(|pattern| smudge_finder(pattern, (0, 0)))
        .map(|pattern| calculate_result(pattern))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
