use itertools::Itertools;

advent_of_code::solution!(3);

fn contains_symbols(string: &str) -> bool {
    string.contains(|c: char| !c.is_digit(10) && c != '.')
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect_vec();

    let mut part_numbers: Vec<u32> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let current_line = lines[i];
        let previous_line = match i > 0 {
            true => Some(lines[i - 1]),
            false => None,
        };
        let next_line = lines.get(i + 1);

        let mut j = 0;
        while j < current_line.len() {
            let current_char = current_line.chars().nth(j).unwrap();

            if current_char.is_digit(10) {
                // z marks the index of the next non-digit character
                let mut z = j + 1;
                while z < current_line.len() {
                    let next_char = current_line.chars().nth(z).unwrap();

                    if next_char.is_digit(10) {
                        z += 1;
                        continue;
                    } else {
                        break;
                    }
                }

                let extracted_number = current_line.chars().skip(j).take(z - j).collect::<String>();
                let extracted_number = extracted_number.parse::<u32>().unwrap();

                let skip = match j > 0 {
                    true => j - 1,
                    false => j,
                };
                let take = match j > 0 {
                    true => z + 2 - j,
                    false => z + 1 - j,
                };
                let characters_above: Option<String> = match previous_line {
                    Some(line) => Some(line.chars().skip(skip).take(take).collect()),
                    None => None,
                };

                let characters_below: Option<String> = match next_line {
                    Some(line) => Some(line.chars().skip(skip).take(take).collect()),
                    None => None,
                };
                let characters_left: String = match j > 0 {
                    true => match current_line.chars().nth(j - 1) {
                        Some(c) => c.to_string(),
                        None => String::new(),
                    },
                    false => String::new(),
                };

                let characters_right: String = match current_line.chars().nth(z) {
                    Some(c) => c.to_string(),
                    None => String::new(),
                };

                j = z;

                let mut matched = false;

                if let Some(above) = characters_above {
                    if contains_symbols(&above) {
                        if !matched {
                            part_numbers.push(extracted_number);
                            matched = true;
                        }
                    }
                }

                if let Some(below) = characters_below {
                    if contains_symbols(&below) {
                        if !matched {
                            part_numbers.push(extracted_number);
                            matched = true;
                        }
                    }
                }

                if contains_symbols(&characters_left) {
                    if !matched {
                        part_numbers.push(extracted_number);
                        matched = true;
                    }
                }

                if contains_symbols(&characters_right) {
                    if !matched {
                        part_numbers.push(extracted_number);
                        matched = true;
                    }
                }
            }

            j += 1;
        }

        i += 1;
    }

    let sum = part_numbers.iter().sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
