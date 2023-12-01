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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
