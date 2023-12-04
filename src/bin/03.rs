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

fn extract_numbers_around_index(input: &str, index: usize) -> Vec<u32> {
    let mut numbers = Vec::new();

    for i in index.saturating_sub(1)..=index + 1 {
        if i < input.len() && input.chars().nth(i).map_or(false, |c| c.is_digit(10)) {
            let mut start = i;
            let mut end = i;

            // walk backwards
            while start > 0
                && input
                    .chars()
                    .nth(start - 1)
                    .map_or(false, |c| c.is_digit(10))
            {
                start -= 1;
            }

            // walk forwards
            while end < input.len() - 1
                && input.chars().nth(end + 1).map_or(false, |c| c.is_digit(10))
            {
                end += 1;
            }

            // parse the number
            if let Ok(number) = input[start..=end].to_string().parse::<u32>() {
                // this is kinda a cheat, but it works
                // the data doesn't put 2 numbers next to each other
                // so this helps with the laziness of for loop above
                if !numbers.contains(&number) {
                    numbers.push(number);
                }
            }
        }
    }

    numbers
}

pub fn part_two(import: &str) -> Option<u32> {
    let lines = import.lines().collect_vec();

    let mut running_sum = 0;

    let mut index_lines = 0;
    while index_lines < lines.len() {
        let current_line = lines[index_lines];
        let previous_line = match index_lines > 0 {
            true => Some(lines[index_lines - 1]),
            false => None,
        };
        let next_line = lines.get(index_lines + 1).map(|line| line.to_owned());

        index_lines += 1;

        // check if the line has a gear, if it doesn't then skip it
        if !current_line.contains('*') {
            continue;
        }

        let chars = current_line.chars().collect_vec();

        let gears = current_line.match_indices('*').collect_vec();

        for (index_gear, _) in gears {
            let mut parts: Vec<u32> = Vec::new();

            // PROCESS ADJACENT: left
            let left = match index_gear > 0 {
                true => Some(chars[index_gear - 1]),
                false => None,
            };
            let left = match left {
                Some(c) => {
                    if c.is_digit(10) {
                        Some(index_gear - 1)
                    } else {
                        None
                    }
                }
                None => None,
            };
            let left = match left {
                Some(index) => Some(extract_numbers_around_index(current_line, index)),
                None => None,
            };
            if let Some(mut numbers) = left {
                parts.append(&mut numbers);
            }

            // PROCESS ADJACENT: below
            let right = match index_gear < current_line.len() - 1 {
                true => Some(chars[index_gear + 1]),
                false => None,
            };
            let right = match right {
                Some(c) => {
                    if c.is_digit(10) {
                        Some(index_gear + 1)
                    } else {
                        None
                    }
                }
                None => None,
            };
            let right = match right {
                Some(index) => Some(extract_numbers_around_index(current_line, index)),
                None => None,
            };
            if let Some(mut numbers) = right {
                parts.append(&mut numbers);
            }

            // PROCESS ADJACENT: above
            if let Some(line) = previous_line {
                let mut numbers = extract_numbers_around_index(&line, index_gear);
                parts.append(&mut numbers);
            }

            // PROCESS ADJACENT: below
            if let Some(line) = next_line {
                let mut numbers = extract_numbers_around_index(&line, index_gear);
                parts.append(&mut numbers);
            }

            if parts.len() == 2 {
                running_sum += parts.iter().product::<u32>();
            }
        }
    }

    Some(running_sum)
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
