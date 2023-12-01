advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: u32 = input
        .lines()
        .map(|line| {
            let number1 = line.chars().find(|c| c.is_digit(10));
            let number2 = line.chars().rfind(|c| c.is_digit(10));

            if let (Some(number1), Some(number2)) = (number1, number2) {
                let combined_number = format!("{}{}", number1, number2);
                combined_number.parse::<u32>().unwrap_or(0)
            } else {
                0
            }
        })
        .sum();

    Some(lines)
}

static MAPPINGS: &[(&str, u32)] = &[
    ("eight", 8),
    ("seven", 7),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("nine", 9),
    ("one", 1),
    ("two", 2),
    ("six", 6),
];

pub fn part_two(input: &str) -> Option<u32> {
    let lines: u32 = input
        .lines()
        .map(|line| {
            let mut digits = [None, None];

            let mut i = 0;
            while i < line.len() {
                let first_char = line.chars().nth(i).unwrap();
                if first_char.is_digit(10) {
                    digits[0] = Some(first_char.to_digit(10).unwrap());
                    break;
                }

                let mut matched = false;
                let mini_line = &line[i..];
                for &(word, digit) in MAPPINGS {
                    if mini_line.starts_with(word) {
                        digits[0] = Some(digit);
                        matched = true;
                        break;
                    }
                }

                if matched {
                    break;
                }

                i += 1;
            }

            let mut i = line.len();
            while i > 0 {
                let last_char_index = i - 1;
                let last_char = line.chars().nth(last_char_index).unwrap();
                if last_char.is_digit(10) {
                    digits[1] = Some(last_char.to_digit(10).unwrap());
                    break;
                }

                let mut matched = false;
                let mini_line = &line[..i];
                for &(word, digit) in MAPPINGS.iter().rev() {
                    if mini_line.ends_with(word) {
                        digits[1] = Some(digit);
                        matched = true;
                        break;
                    }
                }

                if matched {
                    break;
                }

                i -= 1;
            }

            if let (Some(number1), Some(number2)) = (digits[0], digits[1]) {
                let combined_number = format!("{}{}", number1, number2);
                combined_number.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .sum();

    Some(lines)
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
