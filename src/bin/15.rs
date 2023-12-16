use rayon::{prelude::ParallelIterator, str::ParallelString};

advent_of_code::solution!(15);

fn mutate_char(starting: u32, c: char) -> u32 {
    let mut number = starting;
    number += c as u32;
    number *= 17;
    number %= 256;

    number
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .par_split(',')
        .map(|group| group.trim())
        .map(|group| group.chars().fold(0 as u32, mutate_char))
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
        assert_eq!(result, Some(1320));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
